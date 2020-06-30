use super::gpu::*;
// this file contains memory + functions
const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;




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
    bios: [u8; 0x100], //bios (becomes available to rom after boot)
    rom: [u8; 0x8000], //rom
    wram: [u8; 0x2000], //working ram
    eram: [u8; 0x2000], //external ram (On cartridge)
    zram: [u8; 0x80], //zero ram (everything 0xFF80 +)

    ramoffs: u16,
    pub interrupt_flags: u8,
}

impl Memory{
    pub fn new() -> Self{
        Memory {
            gpu: GPU::new(),
            in_bios: true,
            bios: [0x0; 0x100],
            rom: [0x0; 0x8000],
            wram: [0x0; 0x2000],
            eram: [0x0; 0x2000],
            zram: [0x0; 0x80],

            ramoffs: 0,

            interrupt_flags: 0,
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
    pub fn check_gpu(&mut self){
        
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
                        
                        self.in_bios = false;
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
                        if address >= 0xFF80{
                            self.zram[(address & 0x7F)as usize]
                        }else{
                            //io handling go here
                            match address & 0x00FF{
                                0x0 => {
                                    match address & 0x7{
                                        15 => {
                                            return self.interrupt_flags
                                        }
                                        _ => {
                                            return 0
                                        }
                                    }
                                }
                                0x40..=0x4F => {
                                
                                    return self.gpu.rb(address)
                                }
                                0x68..=0x6B => {
                                    
                                    return self.gpu.rb(address)
                                }

                                _ => { return 0 }
                            }
                            0
                        }
                    }
                    _ => { 0 }
                }
            }
            _ => { 0 }
        }
    }
    pub fn wb(&mut self, address: u16, value: u8){
       // println!("{:#x?}",address);
        //TODO - Add memory map
        match address & 0xF000{
            0x0000 => {if self.in_bios{ if address < 0x100 {self.bios[address as usize] = value}else{self.rom[address as usize] = value;}}}
            0x1000..=0x7000 => {self.rom[address as usize] = value;}
            0x8000..=0x9000 => {self.gpu.wb(address, value);}
            0xA000..=0xB000 => {self.eram[(self.ramoffs + (address & 0x1FFF)) as usize] = value;}
            0xC000..=0xE000 => {self.wram[(address & 0x1FFF) as usize] = value;}
            0xF000 => { match address & 0x0F00{
                0x0..=0xD00 => {
                    self.wram[(address & 0x1FFF) as usize] = value;
                }
                0xE00 => {

                    self.gpu.wb(address, value);
                }
                0xF00 => {
                    if address >= 0xFF80{
                        self.zram[(address & 0x7F)as usize] = value;
                    }else{
                        //io handling go here
                        match address & 0x00FF{
                            0x46 => {
                                self.oamdma(value);
                            }
                            0x40..=0x4F => {
                                
                                self.gpu.wb(address, value);
                            }
                            0x68..=0x6B => {
                                
                                self.gpu.wb(address, value);
                            }
                            _ => { 0; }
                        };
                        0;
                    }
                }
                _ => { 0; }
            }  
        }
        _ => { 0; }
    }
}

           
    fn oamdma(&mut self, value: u8) {
    let base = (value as u16) << 8;
    for i in 0 .. 0xA0 {
        let b = self.rb(base + i);
        self.wb(0xFE00 + i, b);
    }
    }       
    pub fn set_bios(&mut self, value: bool){
        self.in_bios = value;
    }
}

