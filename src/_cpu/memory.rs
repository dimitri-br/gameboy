use super::gpu::*;
use std::time;
// this file contains memory + functions
const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub struct Timer{
    divider: u8,
    counter: u8,
    modulo: u8,
    enabled: bool,
    step: u32,
    internal_cnt: u32,
    internal_div: u32,
}
impl Timer{
    pub fn new() -> Self{
        Timer{
            divider: 0,
            counter: 0,
            modulo: 0,
            enabled: false,
            step: 256,
            internal_cnt: 0,
            internal_div: 0,
        }
    }

    pub fn rb(&mut self, addr: u16) -> u8{
        match addr {
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => {
                (if self.enabled { 0x4 } else { 0 }) |
                (match self.step { 16 => 1, 64 => 2, 256 => 3, _ => 0 })
            }
            _ => panic!("Timer does not handler read {:#x?}", addr),
        }
    }
    pub fn wb(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => { self.divider = 0; },
            0xFF05 => { self.counter = val; },
            0xFF06 => { self.modulo = val; }, //tma
            0xFF07 => {
                self.enabled = val & 0x4 != 0;
                self.step = match val & 0x3 { 1 => 16, 2 => 64, 3 => 256, _ => 1024 };
            },
            _ => panic!("Timer does not handler write {:#x?}", addr),
        };
    }
    pub fn do_cycle(&mut self, ticks: u32) -> u8 {
        self.internal_div += ticks; //add cycle count from last instruction
        while self.internal_div >= 256 { //if the internal_div is bigger than 0x100
            self.divider = self.divider.wrapping_add(1); //add to div
            self.internal_div -= 256; //reset
        }

        if self.enabled { //if timer is enabled
            self.internal_cnt += ticks; //add cycle count to internal_cnt

            while self.internal_cnt >= self.step { //step is 256
                self.counter = self.counter.wrapping_add(1); //add to counter. If counter goes over 255, it wraps back to 0
                if self.counter == 0 { //if it is 0
                    self.counter = self.modulo; //set counter to mod
                    return 0x04; //for interrupt
                }
                self.internal_cnt -= self.step; //internal count - 256
            }
            
        }
        return 0x0; //for interrupt
    }
}
pub struct Key{
    pub rows: [u8; 2],
    column: u8,
}
impl Key{
    pub fn new() -> Self{
        Key{
            rows: [0xF; 2],
            column: 0,
        }
    }
    pub fn rb(&self) -> u8{
        match self.column{
            0x10 => {self.rows[0]},
            0x20 => {self.rows[1]},
            _ => {0},
        }
    }
    pub fn wb(&mut self, value: u8){
        self.column = value & 0x30;
    }
}



pub struct MBC{
    rombank: usize,
    rambank: usize,
    ram_on: bool,
    mode: u8,

    rtc_ram: [u8; 5],
    rtc_lock: bool,
    rtc_zero: Option<u64>,
}
impl MBC{
    pub fn new() -> Self{
        MBC{
            rombank: 0,
            rambank: 0,
            ram_on: false,
            mode: 0,

            rtc_ram: [0u8; 5],
            rtc_lock: false,
            rtc_zero: Some(0),
        
        }
    }
}

/// # MEMORY MAP
/// 
/// 0x0000 - 0x3FFF -> 16 kib ROM bank 00
/// 
/// 0x4000 - 0x7FFF -> 16 kib ROM bank 01-NN
/// 
/// 0x8000 - 0x9FFF -> 8 kib VRAM
/// 
/// 0xA000 - 0xBFFF -> 8 kib External RAM
/// 
/// 0xC000 - 0xCFFF -> 4 kib Work bank 0
/// 
/// 0xD000 - 0xDFFF -> 4 kib Work bank 1-N
/// 
/// 0xE000 - 0xFDFF -> (Echo RAM) Mirror of work bank 0xC000 -> 0xDDFF
/// 
/// 0xFE00 - 0xFE9F -> OAM (object attribute memory) Stores Sprites
/// 
/// 0xFEA0 - 0xFEFF -> Non Usable
/// 
/// 0xFF00 - 0xFF7F -> I/O registers
/// 
/// 0xFF80 - 0xFFFE -> High RAM (HRAM)
/// 
/// 0xFFFF - 0xFFFF -> Interrupts 
pub struct Memory{
    pub gpu: GPU,
    pub in_bios: bool,
    pub bios: [u8; 0x100], //bios (becomes available to rom after boot)
    pub rom: Vec::<u8>, //rom
    wram: [u8; 0x2000], //working ram
    eram: [u8; 0x7FFF], //external ram (On cartridge)
    zram: [u8; 0x80], //zero ram (everything 0xFF80 +)

    ramoffs: usize,
    romoffs: usize,

    mbc: MBC,

    pub carttype: u8,

    pub interrupt_flags: u8,
    pub ie: u8,

    pub timer: Timer,
    pub keys: Key,

    pub pc: u16,
}

impl Memory{
    pub fn new() -> Self{
        Memory {
            gpu: GPU::new(),
            in_bios: true,
            bios: [0x0; 0x100],
            rom: Vec::<u8>::new(),
            wram: [0x0; 0x2000],
            eram: [0x0; 0x7FFF],
            zram: [0x0; 0x80],

            ramoffs: 0,
            romoffs: 0x4000,

            mbc: MBC::new(),

            carttype: 0,

            interrupt_flags: 0,
            ie: 0,

            timer: Timer::new(),
            keys: Key::new(),

            pc: 0,
        }
    }
    pub fn set_initial(&mut self) {
        self.wb(0xFF05, 0);
        self.wb(0xFF06, 0);
        self.wb(0xFF07, 0);
        self.wb(0xFF10, 0x80);
        self.wb(0xFF11, 0xBF);
        self.wb(0xFF12, 0xF3);
        self.wb(0xFF14, 0xBF);
        self.wb(0xFF16, 0x3F);
        self.wb(0xFF16, 0x3F);
        self.wb(0xFF17, 0);
        self.wb(0xFF19, 0xBF);
        self.wb(0xFF1A, 0x7F);
        self.wb(0xFF1B, 0xFF);
        self.wb(0xFF1C, 0x9F);
        self.wb(0xFF1E, 0xFF);
        self.wb(0xFF20, 0xFF);
        self.wb(0xFF21, 0);
        self.wb(0xFF22, 0);
        self.wb(0xFF23, 0xBF);
        self.wb(0xFF24, 0x77);
        self.wb(0xFF25, 0xF3);
        self.wb(0xFF26, 0xF1);
        self.wb(0xFF40, 0x91);
        self.wb(0xFF42, 0);
        self.wb(0xFF43, 0);
        self.wb(0xFF45, 0);
        self.wb(0xFF47, 0xFC);
        self.wb(0xFF48, 0xFF);
        self.wb(0xFF49, 0xFF);
        self.wb(0xFF4A, 0);
        self.wb(0xFF4B, 0);
    }

    pub fn rb(&mut self, address: u16) -> u8{
        //TODO - Add memory map
        match address & 0xF000{
            0x0000 => {
                if self.in_bios{
                    if address < 0x100{
                        self.bios[address as usize]
                    }
                    else{
                        
                        self.rom[address as usize]
                    }

                }else{
                    self.rom[address as usize]
                }
            }
            0x1000..=0x3000 => {self.rom[address as usize]}
            0x4000..=0x7000 => {
                self.rom[(self.romoffs + ((address as usize) & 0x3FFF))]
            }
            0x8000..=0x9000 => {
                self.gpu.rb(address)
            }
            0xA000..=0xB000 => {
                if !self.mbc.ram_on { return 0 };
                match self.carttype{
                    1..=3 => {
                        self.eram[(self.ramoffs + ((address as usize) & 0x1FFF))]

                    }
                    0xF..=0x13 => {
                        if self.mbc.rambank <= 3{
                            self.eram[(self.ramoffs + ((address as usize) & 0x1FFF))]
                        }else{
                            self.mbc.rtc_ram[(self.mbc.rambank - 0x8) as usize]
                            
                        }
                    }

                    0x19..=0x1E => {
                        self.eram[(self.ramoffs + ((address as usize) & 0x1FFF))]
                    }
                    _ => {
                        0
                    }
                }
            }
            0xC000..=0xE000 => {
                self.wram[(address & 0x1FFF) as usize]
            }
            0xF000 => {
                match address & 0x0F00{
                    0x0..=0xD00 => {
                        self.wram[(address & 0x1FFF) as usize]
                    }
                    0xE00 => {
                        if address & 0xFF < 0xA0 { self.gpu.rb(address) }else{ 0 }
                        
                    }
                    0xF00 => {
                        if address == 0xFFFF { return self.ie }
                        else if address >= 0xFF80 && address <= 0xFFFE{
                            self.zram[address as usize & 0x7F]
                        }else{
                            //io handling go here
                            match address & 0x00FF{
                                0x0..=0xF => {
                                    match address & 0xF{
                                        0x0 => { return self.keys.rb() } //inp
                                        0x4..=0x7 => { return self.timer.rb(address) as u8 } //timer
                                        0xF => {
   
                                            return self.interrupt_flags
                                        }
                                        _ => {
                                            return 0
                                        }
                                    }
                                }
                                0x1..=0x2 => {
                                    //serial
                                }
                                0x40..=0x4F => {
                                
                                    return self.gpu.rb(address)
                                }
                                0x68..=0x6B => {
                                    
                                    return self.gpu.rb(address)
                                }
                                0x70 => {
                                    return 0
                                }

                                _ => { return 0 }
                            }
                            0
                        }
                    }
                    _ => { panic!("memory: {:#x?}",address) }
                }
            }
            _ => { panic!("memory: {:#x?}",address) }
        }
    }
    pub fn wb(&mut self, address: u16, value: u8){

        //TODO - Add memory map
        match address & 0xF000{
            0x0000..=0x1000 => {
                match self.carttype{
                    1..=3 => {
                        self.mbc.ram_on = value == 0xA;
                        //println!("External RAM: {}", self.mbc.ram_on);

                    }
                    0xF..=0x13 => {
                        self.mbc.ram_on = value == 0xA;
                    }

                    0x19..=0x1E => {
                        self.mbc.ram_on = value == 0xA;
                    }
                    _ => {}
                }
            } //TODO - add MBC
            0x2000..=0x3000 => {
                match self.carttype{
                    1..=3 => {
                        self.mbc.rombank &= 0x60;
                        let mut value = value & 0x1F;
                        if value == 0{
                            value = 1;
                        }
                        self.mbc.rombank |= value as usize;
                        //println!("RomBank {}", self.mbc.rombank);
                        self.romoffs = (self.mbc.rombank as usize) * 0x4000;
                       // println!("romoff -> {}", self.romoffs);
                    }
                    0xF..=0x13 => {
                        self.mbc.rombank = match value & 0x7F { 0 => 1, n => n as usize };
                        self.romoffs = (self.mbc.rombank as usize) * 0x4000;
                    }
                    0x19..=0x1E => {
                        match address & 0xF000{
                            0x2000 => {
                                self.mbc.rombank = (self.mbc.rombank & 0x100) | (value as usize);
                                self.romoffs = (self.mbc.rombank as usize) * 0x4000;
                            }
                            0x3000 => {
                                self.mbc.rombank = (self.mbc.rombank & 0xFF) | (((value & 0x1) as usize) << 8);
                                self.romoffs = (self.mbc.rombank as usize) * 0x4000;
                            }
                            _ => {}
                        }
                        //println!("RomBank {}", self.mbc.rombank);
                        
                    }
                    _ => {}
                }
            }
            0x4000..=0x5000 => {
                match self.carttype{
                    1..=3 => {
                        if self.mbc.mode != 0{
                            self.mbc.rambank = (value as usize) & 3;
//                            println!("RamBank{}", self.mbc.rombank);

                            self.ramoffs = (self.mbc.rambank as usize) * 0x2000;
                        }else{
                            self.mbc.rombank &= 0x1F;
                            self.mbc.rombank |= ((value as usize) & 3) << 5;
                            //println!("RomBank {}", self.mbc.rombank);

                            self.romoffs = (self.mbc.rombank as usize) * 0x4000;
                            //println!("romoff -> {}", self.romoffs);
                        }
                    }
                    0xF..=0x13 => {
                        self.mbc.rambank = value as usize;
                        self.ramoffs = self.mbc.rambank * 0x2000;
                    }
                    0x19..=0x1E => {
                        self.mbc.rambank = (value & 0x0F) as usize;
                        self.ramoffs = self.mbc.rambank * 0x2000;
                        //println!("RamBank {}", self.mbc.rambank);
                    }
                    _ => {}
                }
            }
            0x6000..=0x7000 => {
                match self.carttype{
                    1..=3 => {
                        self.mbc.mode = value & 0x1;
                    }
                    0xF..=0x13 => {
                        match value{
                            0 => {
                                self.mbc.rtc_lock = false
                            }
                            1 => {
                                if !self.mbc.rtc_lock{
                                    let tzero = match self.mbc.rtc_zero {
                                        Some(t) => time::UNIX_EPOCH + time::Duration::from_secs(t),
                                        None => return,
                                    };
                                    if self.mbc.rtc_ram[4] & 0x40 == 0x40 { return }
                            
                                    let difftime = match time::SystemTime::now().duration_since(tzero) {
                                        Ok(n) => { n.as_secs() },
                                        _ => { 0 },
                                    };
                                    self.mbc.rtc_ram[0] = (difftime % 60) as u8;
                                    self.mbc.rtc_ram[1] = ((difftime / 60) % 60) as u8;
                                    self.mbc.rtc_ram[2] = ((difftime / 3600) % 24) as u8;
                                    let days = difftime / (3600*24);
                                    self.mbc.rtc_ram[3] = days as u8;
                                    self.mbc.rtc_ram[4] = (self.mbc.rtc_ram[4] & 0xFE) | (((days >> 8) & 0x01) as u8);
                                    if days >= 512 {
                                        self.mbc.rtc_ram[4] |= 0x80;
                                        if self.mbc.rtc_zero.is_none() { return }
                                        let mut difftime = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
                                            Ok(t) => t.as_secs(),
                                            Err(_) => panic!("System clock is set to a time before the unix epoch."),
                                        };
                                        difftime -= self.mbc.rtc_ram[0] as u64;
                                        difftime -= (self.mbc.rtc_ram[1] as u64) * 60;
                                        difftime -= (self.mbc.rtc_ram[2] as u64) * 3600;
                                        let days = ((self.mbc.rtc_ram[4] as u64 & 0x1) << 8) | (self.mbc.rtc_ram[3] as u64);
                                        difftime -= days * 3600 * 24;
                                        self.mbc.rtc_zero = Some(difftime);
                                    }
                                }
                                self.mbc.rtc_lock = true;
                            }
                            _ => {}

                        }
                    }
                    _ => {}
                }
            }
            0x8000..=0x9000 => {self.gpu.wb(address, value); }
            0xA000..=0xB000 => {
                if !self.mbc.ram_on { return };
                match self.carttype{
                    1..=3 => {
                        self.eram[(self.ramoffs + ((address as usize) & 0x1FFF))] = value;

                    }
                    0xF..=0x13 => {
                        if self.mbc.rambank <= 3{
                            self.eram[(self.ramoffs + ((address as usize) & 0x1FFF))] = value;
                        }else{
                            self.mbc.rtc_ram[(self.mbc.rambank as usize) - 0x8] = value;
                            if self.mbc.rtc_zero.is_none() { return }
                            let mut difftime = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
                                Ok(t) => t.as_secs(),
                                Err(_) => panic!("System clock is set to a time before the unix epoch."),
                            };
                            difftime -= self.mbc.rtc_ram[0] as u64;
                            difftime -= (self.mbc.rtc_ram[1] as u64) * 60;
                            difftime -= (self.mbc.rtc_ram[2] as u64) * 3600;
                            let days = ((self.mbc.rtc_ram[4] as u64 & 0x1) << 8) | (self.mbc.rtc_ram[3] as u64);
                            difftime -= days * 3600 * 24;
                            self.mbc.rtc_zero = Some(difftime);
                        }
                    }

                    0x19..=0x1E => {
                        self.eram[(self.ramoffs + ((address as usize) & 0x1FFF))] = value;
                    }
                    _ => {}
                }
                
            }
            0xC000..=0xE000 => {self.wram[(address & 0x1FFF) as usize] = value;}
            0xF000 => { 
                match address & 0x0F00{
                    0x0..=0xD00 => {
                        self.wram[(address & 0x1FFF) as usize] = value;
                    }
                    0xE00 => {

                        self.gpu.wb(address, value);
                    }
                    0xF00 => {
                        if address == 0xFFFF { /*println!("Wrote '{:#x?}' to IE.... PC: {:#x?}", value, self.pc);*/self.ie = value; }
                        else if address >= 0xFF80 && address < 0xFFFF{
                           /* if address == 0xFFE3 && value == 0x3{
                                println!("FFE3:{:#x?}",value);
                            }else if address == 0xFFE3 && value == 0x2F{
                                panic!("What the fuck do you think you're doing?... PC: {:#x?}", self.pc)
                            }*/
                            self.zram[address as usize & 0x7F] = value;
                        }else{
                            //io handling go here
                            match address & 0x00FF{
                                0x0..=0x0F => {
                                    match address & 0xF{
                                        0x0 => { self.keys.wb(value as u8); } //inp
                                        0x4..=0x7 => { self.timer.wb(address, value as u8);} //timer
                                        0xF => { self.interrupt_flags = value; } //IF
                                        _ => {}
                                    }
                                }
                                0x10..=0x3F => { }//sound
                                0x46 => {
                                    self.oamdma(value);
                                }
                                0x40..=0x4F => {
                                    
                                    self.gpu.wb(address, value);
                                }
                                0x68..=0x6B => {
                                    
                                    self.gpu.wb(address, value);
                                }
                                _ => {  }
                            };
                    }
                }
                _ => { panic!("memory: {:#x?}",address) }
            }  
        }
        _ => { panic!("memory: {:#x?}",address) }
    }
}

           
    fn oamdma(&mut self, value: u8) {
    let base = (value as u16) << 8;
    for i in 0 .. 0xA0 {
        let b = self.rb(base + i);
        self.wb(0xFE00 + i, b);
        }
    }       
}

