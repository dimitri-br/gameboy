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
    pub delay: u32,
    pub pause: bool,
}


//new
impl CPU {
    pub fn new() -> Self{
        CPU { 
            memory: Memory::new(),
            registers: Registers::new(),
            cpu_interrupt: true,
            delay: 4,
            pause: false,
        }
    }
    pub fn load_rom(&mut self){
        let mut rom = ROM::new(String::from("./roms/cpu_instrs/cpu_instrs/individual/06-ld r,r.gb"));
        rom.load();
        let mut index = 0x0;
        for line in rom.content.iter(){
            self.memory.wb(index, *line);
            index += 1;
        }
    }
}

//opcodes
impl CPU {
    pub fn step(&mut self){
        let max_update = 69905;
        self.delay = 0;
        while self.delay < max_update{
            let clockspeed = 4194304;
        
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
            //println!("DEBUG:\nOpcode: {:#x?}\nPC: {:#x?}\nSP: {:#x?}\nA: {:#x?}\nC: {:#x?}\nHL: {:#x?}\nV: {:#x?}\nZ: {}\nCarry: {}",opcode, self.registers.pc, self.registers.sp, self.registers.a, self.registers.c, self.registers.get_hl(), v, self.registers.get_zero(), self.registers.get_carry());
            //println!("PC: {:#x?} - Opcode: {:#x?}", self.registers.pc, opcode);
            self.delay += self.execute(opcode, v) as u32;
            
            self.memory.gpu.do_cycle(self.delay);
        }
        
        
    }

    pub fn execute(&mut self, opcode: u8, v: usize) -> u8{
            match opcode{
                0x0 => {
                    self.registers.pc += 1;
                    4
                }
                0x1 => {
                    self.ld(Target::BC, v);
                    self.registers.pc += 3;
                    12
                }
                0x3 => {
                    self.inc(Target::BC);
                    self.registers.pc += 1;
                    8
                }
                0x4 => {
                    self.inc(Target::B);
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
                    
                    self.memory.wb(v as u16, (self.registers.sp & 0xFF) as u8);
                    self.memory.wb((v + 1)as u16, (self.registers.sp >> 8) as u8);
                    self.registers.pc += 3;
                    20
                }
                0xB => {
                    self.dec(Target::BC);
                    self.registers.pc += 1;
                    8
                }
                0xC => {
                    self.inc(Target::C);
                    self.registers.pc += 1;
                    
                    //panic!("{:#x?} - {:#x?}", self.registers.get_de(), opcode);
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
                    8
                }
                0x13 => {
                    self.inc(Target::DE);
                    self.registers.pc += 1;
                    8
                }
                0x14 => {
                    self.inc(Target::D);
                    self.registers.pc += 1;

                    4
                }
                0x15 => {
                    self.dec(Target::D);
                    self.registers.pc += 1;
                    4
                }
                0x16 => {
                    self.ld(Target::D, v);
                    self.registers.pc += 2;
                    8
                }
                0x17 =>  {
                    self.rla();
                    self.registers.pc += 1;
                    4
                }
                0x18 => {
                    self.registers.pc += 2;
                    let pc = self.registers.pc as i16;
                    let val = v as i8;
                    self.registers.pc = pc.wrapping_add(val as i16) as u16;
                    12
                }
                0x19 => {
                    self.add(Target::HL, self.registers.get_de() as usize);
                    self.registers.pc += 1;
                    8
                }
                0x1A => {
                    let val = self.memory.rb(self.registers.get_de()) as usize;
                    self.ld(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0x1C => {
                    self.inc(Target::E);
                    self.registers.pc += 1;
                    4
                }
                0x1D => {
                    self.dec(Target::E);
                    self.registers.pc += 1;
                    4
                }
                0x1E => {
                    self.ld(Target::E, v);
                    self.registers.pc += 2;
                    8
                }
                0x1F => {
                    self.rra();
                    self.registers.pc += 1;
                    4
                }
                0x20 => {
                    self.registers.pc += 2;
                    
                    if self.registers.get_zero() == false{
                        let pc = self.registers.pc as i16;
                        let val = v as i8;
                        self.registers.pc = pc.wrapping_add(val as i16) as u16;           
                        
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
                0x24 => {
                    self.inc(Target::H);
                    self.registers.pc += 1;
                    4
                }
                0x25 => {
                    self.dec(Target::H);
                    self.registers.pc += 1;
                    4
                }
                0x26 => {
                    self.ld(Target::H, v);
                    self.registers.pc += 2;
                    8
                }
                0x27 => { //DAA
                    if !self.registers.get_sub(){
                        if self.registers.get_carry() || self.registers.a > 0x99{
                            self.registers.a += 0x60;
                            self.registers.set_carry(true);
                        }
                        if self.registers.get_half() || self.registers.a & 0x0F > 0x09{
                            self.registers.a += 0x6;
                        }
                    }else{
                        if self.registers.get_carry(){
                            self.registers.a -= 0x60;
                            self.registers.set_carry(true);
                        }
                        if self.registers.get_half(){
                            self.registers.a -= 0x06;
                        }
                    }
                    self.registers.set_zero(self.registers.a == 0);
                    self.registers.set_half(false);
                    self.registers.pc += 1;
                    
                    4
                }
                0x28 => {
                    self.registers.pc += 2;

                    if self.registers.get_zero() == true{
                        let pc = self.registers.pc as i16;
                        let val = v as i8;
                        self.registers.pc = pc.wrapping_add(val as i16) as u16;
                        12
                    }else{
                        8
                    }
                }
                0x29 => {
                    self.add(Target::HL, self.registers.get_hl() as usize);
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
                0x2C => {
                    self.inc(Target::L);
                    self.registers.pc += 1;
                    4
                }
                0x2D => {
                    self.dec(Target::L);
                    self.registers.pc += 1;
                    4
                }
                0x2E => {
                    self.ld(Target::L, v);
                    self.registers.pc += 2;
                    8
                }
                0x2F => {
                    self.cpl();
                    self.registers.pc += 1;
                    4
                }
                0x30 => {
                    self.registers.pc += 2;
                    if self.registers.get_carry() == false{
                        let pc = self.registers.pc as i16;
                        let val = v as i8;
                        self.registers.pc = pc.wrapping_add(val as i16) as u16;
                        12
                    }else{
                        8
                    }
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
                0x33 => {
                    self.inc(Target::SP);
                    self.registers.pc += 1;
                    8
                }
                0x34 => {
                    let val = self.memory.rb(self.registers.get_hl()) & 0xF;
                    let (new_value, _did_overflow) = val.overflowing_add(1 as u8);
                    self.registers.set_zero(new_value == 0);
                    self.registers.set_sub(false);
                    self.registers.set_half((val & 0xF).overflowing_add((1 as u8) & 0xF).1);                
                    self.memory.wb(self.registers.get_hl(),new_value);
                    self.registers.pc += 1;
                    12
                }
                0x35 => {
                    let val = self.memory.rb(self.registers.get_hl());
                    let new_val = val.wrapping_sub(1);
                    self.registers.set_zero(new_val == 0);
                    self.registers.set_sub(true);
                    self.registers.set_half((val & 0xF).overflowing_sub(1 & 0xF).1);
                    self.memory.wb(self.registers.get_hl(), new_val);
                    self.registers.pc += 1;
                    12
                }
                0x38 => {
                    self.registers.pc += 2;
                    if self.registers.get_carry(){
                        let pc = self.registers.pc as i16;
                        let val = v as i8;
                        self.registers.pc = pc.wrapping_add(val as i16) as u16;
                        12
                    }else{
                        8
                    }
                }
                0x39 => {
                    self.add(Target::HL, self.registers.sp as usize);
                    self.registers.pc += 1;
                    8
                }
                0x3B => {
                    self.dec(Target::SP);
                    self.registers.pc += 1;
                    8
                }
                0x3C => {
                    self.inc(Target::A);
                    self.registers.pc += 1;
                    4
                }
                0x3D => {
                    self.dec(Target::A);
                    self.registers.pc += 1;
                    4
                }
                0x3E => {
                    self.ld(Target::A, v);
                    self.registers.pc += 2;
                    8
                }
                0x42 => {
                    self.ld(Target::B, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x46 => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::B, val);
                    self.registers.pc += 1;
                    8
                }
                0x47 => {
                    self.ld(Target::B, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x4E => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::C, val);
                    self.registers.pc += 1;
                    8
                }
                0x4F => {
                    self.ld(Target::C, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x56 => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::D, val);
                    self.registers.pc += 1;
                    8
                }
                0x57 => {
                    self.ld(Target::D, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x58 => {
                    self.ld(Target::E, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x5D => {
                    self.ld(Target::E, self.registers.l as usize);
                    self.registers.pc += 1;
                    4
                }
                0x5E => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::E, val);
                    self.registers.pc += 1;
                    8
                }
                0x5F => {
                    self.ld(Target::E, self.registers.a as usize);
                    self.registers.pc += 1;

                    4
                }
                0x62 => {
                    self.ld(Target::H, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x66 => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::H, val);
                    self.registers.pc += 1;
                    8
                }
                0x67 => {
                    self.ld(Target::H, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x6B => {
                    self.ld(Target::L, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x6E => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::L, val);
                    self.registers.pc += 1;
                    8
                }
                0x6F => {
                    self.ld(Target::L, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x70 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.b);
                    self.registers.pc += 1;
                    8
                }
                0x71 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.c);
                    self.registers.pc += 1;
                    8
                }
                0x72 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.d);
                    self.registers.pc += 1;
                    8
                }
                0x73 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.e);
                    self.registers.pc += 1;
                    8
                }
                0x77 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.a);
                    self.registers.pc += 1;
                    8
                }
                0x78 => {
                    self.ld(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x79 => {
                    self.ld(Target::A, self.registers.c as usize);
                    self.registers.pc += 1;
                    

                    4
                }
                0x7A => {
                    self.ld(Target::A, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x7B => {
                    self.ld(Target::A, self.registers.e as usize);
                    self.registers.pc += 1;

                    4
                }
                0x7C => {
                    self.ld(Target::A, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x7D => {
                    self.ld(Target::A, self.registers.l as usize);
                    self.registers.pc += 1;

                    4
                }
                0x7E => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::A, val);
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
                0x90 => {
                    self.sub(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;
                    
                    4
                }
                0xA9 => {
                    self.xor(self.registers.c);
                    self.registers.pc += 1;
                    4
                }
                0xAD => {
                    self.xor(self.registers.l);
                    self.registers.pc += 1;
                    4
                }
                0xAE => {
                    let val = self.memory.rb(self.registers.get_hl());
                    self.xor(val);
                    self.registers.pc += 1;
                    8
                }
                0xAF => {
                    self.xor(self.registers.a);
                    self.registers.pc += 1;
                    4
                }
                0xB0 => {
                    self.or(self.registers.b);
                    self.registers.pc += 1;
                    4
                }
                0xB1 => {
                    self.or(self.registers.c);
                    self.registers.pc += 1;
                    4
                }
                0xB6 => {
                    let val = self.memory.rb(self.registers.get_hl());
                    self.or(val);
                    self.registers.pc += 1;
                    8
                }
                0xB7 => {
                    self.or(self.registers.a);
                    self.registers.pc += 1;
                    4
                }
                0xBB => {


                    self.cp(Target::A, self.registers.e as usize);


                    self.registers.pc += 1;
                    4
                }
                0xBE => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.cp(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0xC1 => {
                    self.registers.b = self.memory.rb(self.registers.sp.wrapping_add(1));
                    self.registers.c = self.memory.rb(self.registers.sp);
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    self.registers.pc += 1;

                    12
                }
                0xC2 => {
                    
                    if self.registers.get_zero() == false{
                        self.registers.pc = v as u16;
                        16
                    }else{
                        self.registers.pc += 3;
                        12
                    }
                }
                0xC3 => {
                    self.registers.pc = v as u16;
                    16
                }
                0xC4 => {
                    self.registers.pc += 3;
                    if self.registers.get_zero() == false{
                        self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                        self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                        self.registers.sp = self.registers.sp.wrapping_sub(2);
                        self.registers.pc = v as u16;
                        24
                    }else{
                        12
                    }
                }
                0xC5 => {
                    self.memory.wb(self.registers.sp.wrapping_sub(1), self.registers.b);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), self.registers.c);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc += 1;
                    
                    
                    
                    16
                }
                0xC6 => {

                    self.add(Target::A, v);
                    self.registers.pc += 2;

                    8
                }
                0xC8 => {
                    if self.registers.get_zero(){
                        self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                        self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                        self.registers.sp = self.registers.sp.wrapping_add(2);
                        20
                    }else{
                        self.registers.pc += 1;
                        8
                    }
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
                    //println!("Opcode CB: {:#x?}", val);
                    let delay = self.execute_cb(val, v);
                    self.registers.pc += 1;
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
                0xCE => {
                    self.adc(Target::A, v);
                    self.registers.pc += 2;
                    8
                }
                0xD0 => {
                    
                    if self.registers.get_carry() == false{
                        self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                        self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                        self.registers.sp = self.registers.sp.wrapping_add(2);
                        20
                    }else{
                        self.registers.pc += 1;
                        8
                    }
                }
                0xD1 => {
                    self.registers.d = self.memory.rb(self.registers.sp.wrapping_add(1));
                    self.registers.e = self.memory.rb(self.registers.sp);
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    self.registers.pc += 1;
            

                    12
                }
                0xD5 => {
                    self.memory.wb(self.registers.sp.wrapping_sub(1), self.registers.d);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), self.registers.e);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc += 1;

                    16
                }
                0xD6 => {
                    
                    self.sub(Target::A, v);
                    self.registers.pc += 2;
                    8
                }
                0xD8 => {
                    if self.registers.get_carry(){
                        self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                        self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                        self.registers.sp = self.registers.sp.wrapping_add(2);
                        20
                    }else{
                        self.registers.pc += 1;
                        8
                    }
                }
                0xD9 => {
                    self.cpu_interrupt = true;
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
                    self.memory.wb(0xff00 + (v as u16), self.registers.a);
                    self.registers.pc += 2;
                    12
                }
                0xE1 => {
                    self.registers.h = self.memory.rb(self.registers.sp.wrapping_add(1));
                    self.registers.l = self.memory.rb(self.registers.sp);
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    self.registers.pc += 1;

                    12
                }
                0xE2 => {
                    self.memory.wb(0xff00 + (self.registers.c as u16), self.registers.a);
                    self.registers.pc += 1;
                    8
                }
                0xE5 => {
                    self.memory.wb(self.registers.sp.wrapping_sub(1), self.registers.h);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), self.registers.l);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc += 1;
                    16
                }
                0xE6 => {
                    self.and(v as u8);
                    self.registers.pc += 2;
                    
                    8
                }
                0xE8 => {
                    let new_val = v as i8;
                    self.add(Target::SP, new_val as usize);
                    self.registers.pc += 1;
                    16
                }
                0xE9 => {
                    self.registers.pc = self.registers.get_hl();
                    4
                }
                0xEA => {
                    self.memory.wb(v as u16, self.registers.a);
                    self.registers.pc += 3;
                    16
                }
                0xF0 => {
                    let val = self.memory.rb(0xFF00 + ((v as u8) as u16));
                    self.ld(Target::A, val as usize);
                    self.registers.pc += 2;
                    12
                }
                0xF1 => {
                    
                    self.registers.a = self.memory.rb(self.registers.sp.wrapping_add(1));
  
                    let new_val = self.memory.rb(self.registers.sp);

                    self.registers.set_f(new_val);

                   
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    self.registers.pc += 1;

            
                    12
                }
                0xF3 => {
                    self.cpu_interrupt = false;
                    self.registers.pc += 1;
                    4
                }
                0xF5 => {
                    self.memory.wb(self.registers.sp.wrapping_sub(1), self.registers.a);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), self.registers.get_f());
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc += 1;
                    

                    
                    16
                }
                0xF8 => {
                    let new_val = self.registers.sp as i16;
                    let new_v = (v as i8) as i16;
                    let (nv, overflow) = new_val.overflowing_add(new_v);
                    self.ld(Target::HL, new_val as usize);
                    self.registers.set_zero(false);
                    self.registers.set_sub(false);
                    self.registers.set_carry(overflow);
                    self.registers.set_half((new_val & 0xF).overflowing_add(new_v & 0xF).1);

                    self.registers.pc += 2;
                    12
                }
                0xF9 => {
                    self.ld(Target::SP, self.registers.get_hl() as usize);
                    self.registers.pc += 1;
                    8
                }
                0xFA => {
                    let val = self.memory.rb(v as u16) as usize;
                    self.ld(Target::A, val);
                    self.registers.pc += 3;
                    16
                }
                0xFB => {
                    self.cpu_interrupt = true;
                    self.registers.pc += 1;
                    4
                }
                0xFC => {
                    self.registers.pc += 1;
                    4
                }
                0xFE => {
                    self.cp(Target::A, v);
                    self.registers.pc += 2;
                    8
                }
                0xFF => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 56;
                    16
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
            0x19 => {
                self.rr(Target::C);
                8
            }
            0x1A => {
                self.rr(Target::D);
                8
            }
            0x1B => {
                self.rr(Target::E);
                8
            }
            0x37 => {
                self.swap(Target::A);
                8
            }
            0x38 => {
                self.srl(Target::B);
                8
            }
            0x7C => {
                self.bit(Target::H, 7);
                8
            }
            0x7E => {
                self.bit(Target::HL, 7);
                12
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
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.sp & 0xFFF).overflowing_add((value & 0xFFF) as u16).1);
                self.registers.set_carry(did_overflow);
                
                self.registers.sp = new_value;
            }
            Target::HL => {
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16);

                
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_hl() & 0xFFF).overflowing_add(value as u16 & 0xFFF).1);
                self.registers.set_carry(did_overflow);
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
                self.registers.set_half((self.registers.a & 0xF).overflowing_add(((value + if self.registers.get_carry() { 1 } else { 0 }) as u8) & 0xF).1);
                
                self.registers.a = new_value;
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
            
            _ => {}
        }
    }
    fn and(&mut self, value: u8){    
        let new_value = self.registers.a & value;
        self.registers.set_half(true);
        self.registers.set_zero(new_value == 0);
        self.registers.set_sub(false);

        self.registers.a = new_value;
    }
    fn or(&mut self, value: u8){
        let new_value = self.registers.a | value;
        self.registers.set_zero(new_value == 0);
        self.registers.set_carry(false);
        self.registers.set_half(false);
        self.registers.set_sub(false);
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
        self.registers.set_half(true);
        self.registers.set_sub(true);
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
                self.registers.b ^= 1 << value;
                self.registers.set_zero(self.registers.b == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::C => {
                self.registers.c ^= 1 << value;
                self.registers.set_zero(self.registers.c == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::D => {
                self.registers.d ^= 1 << value;
                self.registers.set_zero(self.registers.d == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::E => {
                self.registers.e ^= 1 << value;
                self.registers.set_zero(self.registers.e == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::H => {
                self.registers.h ^= 1 << value;
                self.registers.set_zero(self.registers.h == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::L => {
                self.registers.l ^= 1 << value;
                self.registers.set_zero(self.registers.l == 0);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },

            Target::HL => {//mem address
                let val =self.memory.rb(self.registers.get_hl()) ^  (1 << value);
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
                let (new_val, did_overflow) = self.registers.a.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.a = new_val;
            },
            Target::B => {
                let (new_val, did_overflow) = self.registers.b.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.b = new_val;
            },
            Target::C => {
                let (new_val, did_overflow) = self.registers.c.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.c = new_val;
            },
            Target::D => {
                let (new_val, did_overflow) = self.registers.d.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.d = new_val;
            },
            Target::E => {
                let (new_val, did_overflow) = self.registers.e.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.e = new_val;
            },
            Target::H => {
                let (new_val, did_overflow) = self.registers.h.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.h = new_val;
            },
            Target::L => {
                let (new_val, did_overflow) = self.registers.l.overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.l = new_val;
            },

            Target::HL => {//mem address
                let (new_val, did_overflow) = self.memory.rb(self.registers.get_hl()).overflowing_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.memory.wb(self.registers.get_hl(), new_val);
            }, 
            _ => {}

        }
    }
    fn rr(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                let (val, overflow) = self.registers.a.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.a = val;
            },
            Target::B => {
                let (val, overflow) = self.registers.b.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.b = val;
            },
            Target::C => {
                let (val, overflow) = self.registers.c.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.c = val;
            },
            Target::D => {
                let (val, overflow) = self.registers.d.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.d = val;
            },
            Target::E => {
                let (val, overflow) = self.registers.e.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.e = val;
            },
            Target::H => {
                let (val, overflow) = self.registers.h.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.h = val;
            },
            Target::L => {
                let (val, overflow) = self.registers.l.overflowing_shr(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let (val, overflow) = self.memory.rb(self.registers.get_hl()).overflowing_shr(1 + self.registers.get_carry() as u32);
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

                let (val, overflow) = self.registers.a.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.a = val;
            },
            Target::B => {
                let (val, overflow) = self.registers.b.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.b = val;
            },
            Target::C => {
                let (val, overflow) = self.registers.c.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.c = val;
            },
            Target::D => {
                let (val, overflow) = self.registers.d.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.d = val;
            },
            Target::E => {
                let (val, overflow) = self.registers.e.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.e = val;
            },
            Target::H => {
                let (val, overflow) = self.registers.h.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.h = val;
            },
            Target::L => {
                let (val, overflow) = self.registers.l.overflowing_shl(1 + self.registers.get_carry() as u32);
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(overflow);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let (val, overflow) = self.memory.rb(self.registers.get_hl()).overflowing_shl(1 + self.registers.get_carry() as u32);
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
    fn rra(&mut self){

        let (val, overflow) = self.registers.a.overflowing_shr(1 + self.registers.get_carry() as u32);
        self.registers.set_zero(false);
        self.registers.set_half(false);
        self.registers.set_sub(false);
        self.registers.set_carry(overflow);
        self.registers.a = val;
    }
    fn swap(&mut self, target: Target){
        match target{
            Target::A => {
                let new_val = (self.registers.a & 0xF0) >> 4 | (self.registers.a & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.a = new_val;
            }
            Target::B => {
                let new_val = (self.registers.b & 0xF0) >> 4 | (self.registers.b & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.b = new_val;
            }
            Target::C => {
                let new_val = (self.registers.c & 0xF0) >> 4 | (self.registers.c & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.c = new_val;
            }
            Target::D => {
                let new_val = (self.registers.d & 0xF0) >> 4 | (self.registers.d & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.d = new_val;
            }
            Target::E => {
                let new_val = (self.registers.e & 0xF0) >> 4 | (self.registers.e & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.e = new_val;
            }
            Target::H => {
                let new_val = (self.registers.h & 0xF0) >> 4 | (self.registers.h & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.h = new_val;
            }
            Target::L => {
                let new_val = (self.registers.l & 0xF0) >> 4 | (self.registers.l & 0xF) << 4;
                self.registers.set_carry(false);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_zero(new_val == 0);
                self.registers.l = new_val;
            }
            Target::HL => {
                    let val = self.memory.rb(self.registers.get_hl());
                    let new_val = (val & 0xF0) >> 4 | (val & 0xF) << 4;
                    self.registers.set_carry(false);
                    self.registers.set_half(false);
                    self.registers.set_sub(false);
                    self.registers.set_zero(new_val == 0);
                    self.memory.wb(self.registers.get_hl(), new_val);
                }
            _ => {}
        }
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