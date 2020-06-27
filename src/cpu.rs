use crate::_cpu::{memory::*, read_rom::*, registers::*, gpu::*};


const OPCODE_LENGTHS : [u8; 512] = [
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1,
    1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1,
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1,
    2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

pub struct CPU{
    pub memory: Memory,
    pub registers: Registers,
    pub cpu_interrupt: bool,
}


//new
impl CPU {
    pub fn new() -> Self{
        CPU { 
            memory: Memory::new(),
            registers: Registers::new(),
            cpu_interrupt: true,
        }
    }
    pub fn load_rom(&mut self){
        let mut rom = ROM::new(String::from("./roms/boot.bin"));
        rom.load();
        let mut index = 0x100;
        for line in rom.content.iter(){
            self.memory.wb(index, *line);
            index += 1;
        }
    }
}

//opcodes
impl CPU {
    pub fn step(&mut self){
        if self.memory.in_bios && self.registers.pc >= 0x100{
            self.memory.in_bios = false;
        }
        let opcode = self.memory.rb(self.registers.pc);
        let opcode_length = OPCODE_LENGTHS[opcode as usize];
        let mut v : usize = 0;
        if opcode_length == 2{
            v = self.memory.rb(self.registers.pc + 1) as usize;
        }
        if opcode_length == 3{
            let a = self.memory.rb(self.registers.pc + 2) as usize;
            let b = self.memory.rb(self.registers.pc + 1) as usize;
            v = (a << 8) + b;
        }
        println!("DEBUG:\nOpcode: {:#x?}\nPC: {}\nSP: {}\nA: {}\nC: {}\nHL: {}\nV: {}",opcode, self.registers.pc, self.registers.sp, self.registers.a, self.registers.c, self.registers.get_hl(), v);
        let delay = self.execute(opcode, v);

        let IF = self.memory.gpu.step(delay);
        self.memory.interrupt_flags |= IF;
    }

    pub fn execute(&mut self, opcode: u8, v: usize) -> u8{
            match opcode{
                0x0 => {
                    self.registers.pc += 1;
                    4
                }
                0x5 => {
                    self.dec(Target::B);
                    self.registers.pc += 1;
                    4
                }
                0x6 => {
                    self.ld(Target::B, v);
                    self.registers.pc += 2;
                    8
                }
                0x8 => {
                    self.memory.wb(v as u16, (self.registers.sp >> 8) as u8);
                    self.memory.wb(((v + 1) as u16), (self.registers.sp & 0xFF) as u8);
                    self.registers.pc += 3;
                    20
                }
                0xC => {
                    self.inc(Target::C);
                    self.registers.pc += 1;
                    4
                }
                0xD => {
                    self.dec(Target::C);
                    self.registers.pc += 1;
                    4
                }
                0xE => {
                    self.ld(Target::C, v);
                    self.registers.pc += 2;
                    8
                }
                0x11 => {
                    self.ld(Target::DE, v);
                    self.registers.pc += 3;
                    12
                }
                0x12 => {
                    self.memory.wb(self.registers.get_de(), self.registers.a);
                    self.registers.pc += 1;
                    16
                }
                0x14 => {
                    self.inc(Target::D);
                    self.registers.pc += 1;
                    4
                }
                0x17 =>  {
                    self.rla();
                    self.registers.pc += 1;
                    4
                }
                0x1A => {
                    let val = self.memory.rb(self.registers.get_de()) as usize;
                    self.ld(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0x1C => {
                    self.inc(Target::C);
                    self.registers.pc += 1;
                    4
                }
                0x20 => {
                    self.registers.pc += 2;
                    if !self.registers.get_zero(){
                        let new_val = v as i8;
                        let pc = self.registers.pc as i16;
                        let new_val = pc.wrapping_add(new_val as i16);
                        self.registers.pc = new_val as u16;
                        12
                    }else{
                        8
                    }
                }
                0x21 => {
                    self.ld(Target::HL, v);
                    self.registers.pc += 3;
                    12
                }
                0x22 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.a);
                    self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                    self.registers.pc += 1;
                    8
                }
                0x23 => {
                    self.inc(Target::HL);
                    self.registers.pc += 1;
                    8
                }
                0x2A => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::A, val);
                    self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                    self.registers.pc += 1;
                    8
                }
                0x31 => {
                    self.ld(Target::SP, v);
                    self.registers.pc += 3;
                    12
                }
                0x32 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.a);
                    self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                    self.registers.pc += 1;
                    8
                }
                0x3C => {
                    self.inc(Target::A);
                    self.registers.pc += 1;
                    4
                }
                0x3E => {
                    self.ld(Target::A, v);
                    self.registers.pc += 2;
                    8
                }
                0x47 => {
                    self.ld(Target::B, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x4F => {
                    self.ld(Target::C, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x77 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.a);
                    self.registers.pc += 1;
                    8
                }
                0x88 => {
                    self.adc(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x89 => {
                    self.adc(Target::A, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0xAF => {
                    self.xor(self.registers.a);
                    self.registers.pc += 1;
                    4
                }
                0xC1 => {
                    self.registers.b = self.memory.rb(self.registers.sp.wrapping_add(2));
                    self.registers.c = self.memory.rb(self.registers.sp.wrapping_add(1));
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    self.registers.pc += 1;
                    16
                }
                0xC3 => {
                    self.registers.pc = v as u16;
                    16
                }
                0xC5 => {
                    self.memory.wb(self.registers.sp.wrapping_sub(1), self.registers.b);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), self.registers.c);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc += 1;
                    16
                }
                0xC6 => {
                    self.add(Target::A, v as usize);
                    self.registers.pc += 2;
                    8
                }
                0xC9 => {
                    self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                    self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    16
                }
                0xCB => {
                    self.registers.pc += 1;
                    let val = self.memory.rb(self.registers.pc);
                    let delay = self.execute_cb(val, v);
                    self.registers.pc += 2;
                    4 + delay
                }
                0xCC => {
                    self.registers.pc += 3;
                    if self.registers.get_zero(){
                        self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                        self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                        self.registers.sp = self.registers.sp.wrapping_sub(2);
                        self.registers.pc = v as u16;
                        24
                    }else{
                        12
                    }
                }
                0xCD => {
                    self.registers.pc += 3;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = v as u16;
                    24
                }
                0xD9 => {
                    self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                    self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    16
                }
                0xDD => {
                    self.registers.pc += 1;
                    4
                }
                0xE0 => {
                    self.memory.wb(((v as u8) as u16) + 0xff00, self.registers.a);
                    self.registers.pc += 2;
                    12
                }
                0xE2 => {
                    self.memory.wb(0xff00 + (self.registers.c as u16), v as u8);
                    self.registers.pc += 1;
                    8
                }
                0xFB => {
                    self.cpu_interrupt = true;
                    self.registers.pc += 1;
                    4
                }
                
                _ => { panic!("Opcode: {:#x?}", opcode) }
            }
    }
}


//cb
impl CPU{
    pub fn execute_cb(&mut self, opcode: u8, v: usize) -> u8{
        match opcode{
            0x11 => {
                self.rl(Target::C);
                8
            }
            0x7C => {
                self.set(Target::H, 7);
                self.registers.set_half(true);
                8
            }
            _ => { panic!("Opcode: CB -> {:#x?}", opcode)}
        }
    }
}


//ALU functions
impl CPU {
    fn add(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.a & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.b & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.c & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.d & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.e & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.e = new_value;
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.h & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.l & 0xF).overflowing_add((value & 0xF) as u8).1);
                
                self.registers.l = new_value;
            }
            Target::SP => {
                let (new_value, did_overflow) = self.registers.sp.overflowing_add(value as u16);

                self.registers.set_zero(false);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.sp & 0xFFF).overflowing_add((value & 0xFFF) as u16).1);
                
                self.registers.sp = new_value;
            }
            Target::HL => {
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16);

                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_hl() & 0xFFF).overflowing_add(value as u16 & 0xFFF).1);
                
                self.registers.set_hl(new_value);
            }
            Target::DE => {
                let (new_value, did_overflow) = self.registers.get_de().overflowing_add(value as u16);


                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_de() & 0xFFF).overflowing_add(value as u16 & 0xFFF).1);
                
                self.registers.set_de(new_value);
            }
            Target::BC => {
                let (new_value, did_overflow) = self.registers.get_bc().overflowing_add(value as u16);


                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_bc() & 0xFFF).overflowing_add(value as u16 & 0xFFF).1);
                
                self.registers.set_bc(new_value);
            }
            _ => {}
        }
    }
    fn sub(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.a & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.b & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.c & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.d & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.e & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.e = new_value;
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.h & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.l & 0xF).overflowing_sub((value & 0xF) as u8).1);
                
                self.registers.l = new_value;
            }
            _ => {}
        }
    }
    fn adc(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.a & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.b & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.c & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.d & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.e & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);                
                self.registers.e = new_value;
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.h & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_add(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.l & 0xF).overflowing_add((value + if self.registers.get_carry() { 1 } else { 0 }) as u8 & 0xF).1);                
                self.registers.l = new_value;
            }
            Target::SP => {
                let (new_value, did_overflow) = self.registers.sp.overflowing_add(value as u16 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.sp & 0xFFF).overflowing_add((value as u16 + if self.registers.get_carry() { 1 } else { 0 }) as u16 & 0xFFF).1);                
                self.registers.sp = new_value;
            }
            Target::HL => {
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_hl() & 0xFFF).overflowing_add((value as u16 + if self.registers.get_carry() { 1 } else { 0 }) as u16 & 0xFFF).1);                
                
                self.registers.set_hl(new_value);
            }
            Target::DE => {
                let (new_value, did_overflow) = self.registers.get_de().overflowing_add(value as u16 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_de() & 0xFFF).overflowing_add((value as u16 + if self.registers.get_carry() { 1 } else { 0 }) as u16 & 0xFFF).1);                
                
                self.registers.set_de(new_value);
            }
            Target::BC => {
                let (new_value, did_overflow) = self.registers.get_bc().overflowing_add(value as u16 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_bc() & 0xFFF).overflowing_add((value as u16 + if self.registers.get_carry() { 1 } else { 0 }) as u16 & 0xFFF).1);                
                
                self.registers.set_bc(new_value);
            }
            _ => {}
        }
    }
    fn sbc(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.a & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.b & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.c & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.d & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.e & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);                
                self.registers.e = new_value;
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.h & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_sub(value as u8 + if self.registers.get_carry() { 1 } else { 0 });

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.l & 0xF).overflowing_sub((value as u8 + if self.registers.get_carry() { 1 } else { 0 }) & 0xF).1);                
                self.registers.l = new_value;
            }
            _ => {}
        }
    }
    fn and(&mut self, value: u8){    
        let new_value = self.registers.a & value;
        self.registers.set_half((self.registers.a & 0xF) & (value & 0xF) > 0xF);
        self.registers.set_zero(new_value == 0);
    }
    fn or(&mut self, value: u8){
        let new_value = self.registers.a | value;
        self.registers.set_zero(new_value == 0);
        self.registers.a = new_value;
    }
    fn xor(&mut self, value: u8){
        let new_value = self.registers.a ^ value;
        self.registers.set_zero(new_value == 0);
        self.registers.set_sub(false);
        self.registers.set_carry(false);
        self.registers.set_half(false);
        self.registers.a = new_value;
    }
    fn cp(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.a & 0xF).overflowing_sub(value as u8 & 0xF).1);

            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.b & 0xF).overflowing_sub(value as u8 & 0xF).1);
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.c & 0xF).overflowing_sub(value as u8 & 0xF).1);
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.d & 0xF).overflowing_sub(value as u8 & 0xF).1);
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.e & 0xF).overflowing_sub(value as u8 & 0xF).1);                
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.h & 0xF).overflowing_sub(value as u8 & 0xF).1);
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.l & 0xF).overflowing_sub(value as u8 & 0xF).1);
            }
            _ => {}
        }
    }
    fn inc(&mut self, target: Target){
        let value = 1;
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);

                self.registers.set_sub(false);
                self.registers.set_half((self.registers.a & 0xF).overflowing_add(value as u8 & 0xF).1);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.b & 0xF).overflowing_add(value as u8 & 0xF).1);                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.c & 0xF).overflowing_add(value as u8 & 0xF).1);                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.d & 0xF).overflowing_add(value as u8 & 0xF).1);                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.e & 0xF).overflowing_add(value as u8 & 0xF).1);                
                self.registers.e = new_value;
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.h & 0xF).overflowing_add(value as u8 & 0xF).1);                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.l & 0xF).overflowing_add(value as u8 & 0xF).1);                
                self.registers.l = new_value;
            }
            Target::SP => {
                let (new_value, did_overflow) = self.registers.sp.overflowing_add(value as u16);

                self.registers.sp = new_value;
            }
            Target::HL => {
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16);

                
                self.registers.set_hl(new_value);
            }
            Target::DE => {
                let (new_value, did_overflow) = self.registers.get_de().overflowing_add(value as u16);


                
                self.registers.set_de(new_value);
            }
            Target::BC => {
                let (new_value, did_overflow) = self.registers.get_bc().overflowing_add(value as u16);


                
                self.registers.set_bc(new_value);
            }
            _ => {}
        }
    }
    fn dec(&mut self, target: Target){
        let value = 1;
        match target{
            Target::A => {
                let (new_value, did_overflow) = self.registers.a.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.a & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.b & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.c & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.d & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.e & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.e = new_value;
            }
            Target::F => {
                
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.h & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.h & 0xF).overflowing_sub(value as u8 & 0xF).1);
                
                self.registers.l = new_value;
            }
            Target::SP => {
                let (new_value, did_overflow) = self.registers.sp.overflowing_sub(value as u16);


                
                self.registers.sp = new_value;
            }
            Target::HL => {
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_sub(value as u16);


                
                self.registers.set_hl(new_value);
            }
            Target::DE => {
                let (new_value, did_overflow) = self.registers.get_de().overflowing_sub(value as u16);

                
                
                self.registers.set_de(new_value);
            }
            Target::BC => {
                let (new_value, did_overflow) = self.registers.get_bc().overflowing_sub(value as u16);


                
                self.registers.set_bc(new_value);
            }
            _ => {}
        }
    }
    fn ccf(&mut self){
        self.registers.set_carry(!self.registers.get_carry());
        self.registers.set_sub(false);
        self.registers.set_half(false);
    }
    fn scf(&mut self){
        self.registers.set_carry(true);
        self.registers.set_sub(false);
        self.registers.set_half(false);
    }
    fn cpl(&mut self){
        self.registers.a ^= 0xFF;
    }
    fn bit(&mut self, target: Target, value: u8){ //value is bit number to toggle (0 - 7)
        match target{
            Target::A => {
                self.registers.a ^= 1 << value;
                self.registers.set_zero(self.registers.a == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::B => {
                self.registers.b ^= value;
                self.registers.set_zero(self.registers.b == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::C => {
                self.registers.c ^= value;
                self.registers.set_zero(self.registers.c == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::D => {
                self.registers.d ^= value;
                self.registers.set_zero(self.registers.d == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::E => {
                self.registers.e ^= value;
                self.registers.set_zero(self.registers.e == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::H => {
                self.registers.h ^= value;
                self.registers.set_zero(self.registers.h == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::L => {
                self.registers.l ^= value;
                self.registers.set_zero(self.registers.l == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },

            Target::HL => {//mem address
                let val =self.memory.rb(self.registers.get_hl()) ^ value;
                self.memory.wb(self.registers.get_hl(), val);
                self.registers.set_zero(self.memory.rb(self.registers.get_hl()) == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            }, 
            _ => {}

        }
    }
    fn reset(&mut self, target: Target, value: u8){ //where value is bit number to reset (0 - 7)
        match target{
            Target::A => {
                self.registers.a &= !(1 << value);
            },
            Target::B => {
                self.registers.b &= !(1 << value);
            },
            Target::C => {
                self.registers.c &= !(1 << value);
            },
            Target::D => {
                self.registers.d &= !(1 << value);
            },
            Target::E => {
                self.registers.e &= !(1 << value);
            },
            Target::H => {
                self.registers.h &= !(1 << value);
            },
            Target::L => {
                self.registers.l &= !(1 << value);
            },

            Target::HL => {//mem address
                let val = self.memory.rb(self.registers.get_hl()) & !(1 << value);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn set(&mut self, target: Target, value: u8){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                self.registers.a |= 1 << value;
            },
            Target::B => {
                self.registers.b |= 1 << value;
            },
            Target::C => {
                self.registers.c |= 1 << value;
            },
            Target::D => {
                self.registers.d |= 1 << value;
            },
            Target::E => {
                self.registers.e |= 1 << value;
            },
            Target::H => {
                self.registers.h |= 1 << value;
            },
            Target::L => {
                self.registers.l |= 1 << value;
            },

            Target::HL => {//mem address
                let val = self.memory.rb(self.registers.get_hl()) | 1 << value;
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn srl(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                self.registers.a >>= 1;
                self.registers.set_zero(self.registers.a == 0);
                self.registers.set_carry(self.registers.a > 0xFF);
            },
            Target::B => {
                self.registers.b >>= 1;
                self.registers.set_zero(self.registers.b == 0);
                self.registers.set_carry(self.registers.b > 0xFF);
            },
            Target::C => {
                self.registers.c >>= 1;
                self.registers.set_zero(self.registers.c == 0);
                self.registers.set_carry(self.registers.c > 0xFF);
            },
            Target::D => {
                self.registers.d >>= 1;
                self.registers.set_zero(self.registers.d == 0);
                self.registers.set_carry(self.registers.d > 0xFF);
            },
            Target::E => {
                self.registers.e >>= 1;
                self.registers.set_zero(self.registers.e == 0);
                self.registers.set_carry(self.registers.e > 0xFF);
            },
            Target::H => {
                self.registers.h >>= 1;
                self.registers.set_zero(self.registers.h == 0);
                self.registers.set_carry(self.registers.h > 0xFF);
            },
            Target::L => {
                self.registers.l >>= 1;
                self.registers.set_zero(self.registers.l == 0);
                self.registers.set_carry(self.registers.l > 0xFF);
            },

            Target::HL => {//mem address
                let val =self.memory.rb(self.registers.get_hl()) >> 1;
                self.memory.wb(self.registers.get_hl(), val);
                self.registers.set_zero(self.memory.rb(self.registers.get_hl()) == 0);
                self.registers.set_carry(self.memory.rb(self.registers.get_hl()) > 0xFF);
            }, 
            _ => {}

        }
    }
    fn rr(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                let (val, overflow) = self.registers.a.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.a = val;
            },
            Target::B => {
                let (val, overflow) = self.registers.b.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.b = val;
            },
            Target::C => {
                let (val, overflow) = self.registers.c.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.c = val;
            },
            Target::D => {
                let (val, overflow) = self.registers.d.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.d = val;
            },
            Target::E => {
                let (val, overflow) = self.registers.e.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.e = val;
            },
            Target::H => {
                let (val, overflow) = self.registers.h.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.h = val;
            },
            Target::L => {
                let (val, overflow) = self.registers.l.overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let (val, overflow) = self.memory.rb(self.registers.get_hl()).overflowing_shr(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn rl(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {

                let (val, overflow) = self.registers.a.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.a = val;
            },
            Target::B => {
                let (val, overflow) = self.registers.b.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.b = val;
            },
            Target::C => {
                let (val, overflow) = self.registers.c.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.c = val;
            },
            Target::D => {
                let (val, overflow) = self.registers.d.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.d = val;
            },
            Target::E => {
                let (val, overflow) = self.registers.e.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.e = val;
            },
            Target::H => {
                let (val, overflow) = self.registers.h.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.h = val;
            },
            Target::L => {
                let (val, overflow) = self.registers.l.overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let (val, overflow) = self.memory.rb(self.registers.get_hl()).overflowing_shl(1);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn rla(&mut self){

        let (val, overflow) = self.registers.a.overflowing_shl(1 + self.registers.get_carry() as u32);
        self.registers.set_zero(false);
        self.registers.set_half(false);
        self.registers.set_sub(false);
        self.registers.set_carry(overflow);
        self.registers.a = val;
    }
}

//gb functions
impl CPU{
    pub fn ld(&mut self, target: Target, value: usize){
        match target{
            Target::A => {
                self.registers.a = value as u8;
            }
            Target::B => {
                self.registers.b = value as u8;
            }
            Target::C => {
                self.registers.c = value as u8;
            }
            Target::D => {
                self.registers.d = value as u8;
            }
            Target::E => {
                self.registers.e = value as u8;
            }
            Target::F => {
                //self.registers.f = value as u8;
            }
            Target::H => {
                self.registers.h = value as u8;
            }
            Target::L => {
                self.registers.l = value as u8;
            }
            Target::SP => {
                self.registers.sp = value as u16;
            }
            Target::HL => {
                self.registers.set_hl(value as u16);
            }
            Target::DE => {
                self.registers.set_de(value as u16);
            }
            Target::AF => {
                self.registers.set_af(value as u16);
            }
            Target::BC => {
                self.registers.set_bc(value as u16);
            }
        }
    }
}

pub enum Target{
    //8-bit
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,

    //16-bit
    AF,
    BC,
    DE,
    HL,
    SP
}