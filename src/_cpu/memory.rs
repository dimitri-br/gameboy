use super::gpu::*;
// this file contains memory + functions
const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub struct Timer{
    main: u16,
    sub: u16,
    div: u16,

    reg_div: u16,
    tima: u16,
    tma: u16,
    tac: u16,
}
impl Timer{
    pub fn new() -> Self{
        Timer {
            main: 0,
            sub: 0,
            div: 0,

            reg_div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn inc(&mut self, m: u16) -> u16{
        self.sub += m;

        if self.sub >= 4{
            self.main += 1;
            self.sub -= 4;


            self.div += 1;
            if self.div == 16{
                self.reg_div = (self.reg_div + 1) & 255;
                self.div = 0;
            }
        }
        self.check()
    }

    pub fn check(&mut self) -> u16{

        if self.tac & 4 != 0{
            
            let threshold = match self.tac & 3{
                0 => {64}
                1 => {1}
                2 => {4}
                3 => {16}
                _ => {panic!()}
            };
            if self.main >= threshold {return self.step()};
        }
        0
    }

    pub fn step(&mut self) -> u16{
        self.main = 0;
        self.tima += 1;
        if self.tima > 255{
            self.tima = self.tma;
            return 4
        }
        return 0
    }

    pub fn rb(&mut self, addr: u16) -> u16{
        match addr{
            0xFF04 => {self.reg_div}
            0xFF05 => {self.tima}
            0xFF06 => {self.tma}
            0xFF07 => {self.tac}
            _ => {0}
        }
    }

    pub fn wb(&mut self, addr: u16, val: u16){
        match addr{
            0xFF04 => {self.reg_div = 0;}
            0xFF05 => {self.tima = val;}
            0xFF06 => {self.tma = val;}
            0xFF07 => {self.tac = val & 7;}
            _ => {0;}
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
    pub rom: [u8; 0xFFFF], //rom
    wram: [u8; 0x2000], //working ram
    eram: [u8; 0x2000], //external ram (On cartridge)
    zram: [u8; 0x80], //zero ram (everything 0xFF80 +)

    ramoffs: u16,
    pub interrupt_flags: u8,
    pub ie: u8,

    pub timer: Timer,
}

impl Memory{
    pub fn new() -> Self{
        Memory {
            gpu: GPU::new(),
            in_bios: true,
            bios: [0x0; 0x100],
            rom: [0xFF; 0xFFFF],
            wram: [0x0; 0x2000],
            eram: [0x0; 0x2000],
            zram: [0x0; 0x80],

            ramoffs: 0,

            interrupt_flags: 0,
            ie: 0,

            timer: Timer::new(),
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
                    else if address >= 0x100{
                        
                        self.rom[address as usize]
                    }
                    else{
                        0
                    }
                }else{
                    self.rom[address as usize]
                }
            }
            0x1000..=0x7000 => {self.rom[address as usize]}
            0x8000..=0x9000 => {
                self.gpu.rb(address)
            }
            0xA000..=0xB000 => {
                self.eram[(self.ramoffs + (address & 0x1FFF)) as usize]
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
                            self.zram[(address & 0x7F)as usize]
                        }else{
                            //io handling go here
                            match address & 0x00FF{
                                0x0 => {
                                    match address & 0x7{
                                        0x0 => { } //inp
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
       // println!("{:#x?}",address);
        //TODO - Add memory map
        match address & 0xF000{
            0x0000 => {
                self.rom[address as usize] = value;
            } //TODO - add MBC
            0x1000..=0x7000 => {
                self.rom[address as usize] = value;
            }
            0x8000..=0x9000 => {self.gpu.wb(address, value);}
            0xA000..=0xB000 => {self.eram[(self.ramoffs + (address & 0x1FFF)) as usize] = value;}
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
                        if address == 0xFFFF { self.ie = value; }
                        else if address >= 0xFF80{
                            self.zram[(address & 0x7F)as usize] = value;
                        }else{
                            //io handling go here
                            match address & 0x00FF{
                                0x0 => {
                                    match address & 0xF{
                                        0x0 => { } //inp
                                        0x4..=0x7 => { self.timer.wb(address, value as u16);} //timer
                                        0xF => { self.interrupt_flags = value; }
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

