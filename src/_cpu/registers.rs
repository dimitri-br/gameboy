// this file contains the registers + functions

pub struct Registers{
    pub a: u8, // Accumulator
    pub f: Flags, // Flag (NOTE: See Flag Function for more info)

    pub b: u8, // Higher of BC
    pub c: u8, // Lower of BC

    pub d: u8, // Higher of DE
    pub e: u8, // Lower of DE

    pub h: u8, // Higher of HL - NOTE: HL can be used as a memory pointer as well as a store
    pub l: u8, // Lower of HL - See note at H

    pub pc: u16, // Program Counter - Stores current memory location to be fetched
    pub sp: u16, // Stack Pointer

}
pub struct Flags{
    zero: bool,
    sub: bool,
    half: bool,
    carry: bool
}
impl Registers{
    pub fn new() -> Self{
        Registers {
            a: 0,
            f: Flags { zero: false, sub: false, half: false, carry: false },
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0x100, 
            sp: 0,
        }
    }


    /// # get_af
    /// 
    /// function will get BC, return type `u16`
    pub fn get_af(&mut self) -> u16{
        let mut f = 0;
        if self.get_carry(){
            f |= 0x10;
        }
        if self.get_zero(){
            f |= 0x80;
        }
        if self.get_half(){
            f |= 0x20;
        }
        if self.get_sub(){
            f |= 0x40;
        }
        (self.a as u16) << 8  | f as u16
    }

    pub fn get_f(&mut self) -> u8{
        let mut f = 0;
        if self.get_carry(){
            f |= 0x10;
        }
        if self.get_zero(){
            f |= 0x80;
        }
        if self.get_half(){
            f |= 0x20;
        }
        if self.get_sub(){
            f |= 0x40;
        }
        f
    }
    pub fn set_f(&mut self, value: u8){
        let new_val = value & 0xF0;
        println!("{}",new_val);
        if new_val & 0x80 == 0x80{
            self.f.zero = true;
        }else{
            self.f.zero = false;
        }
        if new_val & 0x40 == 0x40{
            self.f.sub = true;
        }else{
            self.f.sub = false;
        }
        if new_val & 0x20 == 0x20{
            self.f.half = true;
        }else{
            self.f.half = false;
        }
        if new_val & 0x10 == 0x10{
            self.f.carry = true;
        }else{
            self.f.carry = false;
        }

    }

    /// # set_af
    /// 
    /// function will set BC to value, type `u16`
    pub fn set_af(&mut self, value: u16){
        self.a = ((value & 0xFF00) >> 8) as u8;
        let new_val = value & 0xF0;
        if new_val & 0x80 == 0x80{
            self.f.zero = true;
        }else{
            self.f.zero = false;
        }
        if new_val & 0x40 == 0x40{
            self.f.sub = true;
        }else{
            self.f.sub = false;
        }
        if new_val & 0x20 == 0x20{
            self.f.half = true;
        }else{
            self.f.half = false;
        }
        if new_val & 0x10 == 0x10{
            self.f.carry = true;
        }else{
            self.f.carry = false;
        }
    }

    /// # get_bc
    /// 
    /// function will get BC, return type `u16`
    pub fn get_bc(&self) -> u16{
        (self.b as u16) << 8  | self.c as u16
    }

    /// # set_bc
    /// 
    /// function will set BC to value, type `u16`
    pub fn set_bc(&mut self, value: u16){
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    /// # get_de
    /// 
    /// function will get DE, return type `u16`
    pub fn get_de(&self) -> u16{
        (self.d as u16) << 8  | self.e as u16
    }

    /// # set_de
    /// 
    /// function will set DE to value, type `u16`
    pub fn set_de(&mut self, value: u16){
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    /// # get_hl
    /// 
    /// function will get HL, return type `u16`
    pub fn get_hl(&self) -> u16{
        (self.h as u16) << 8 | self.l as u16
    }

    /// # set_hl
    /// 
    /// function will set HL to value, type `u16`
    pub fn set_hl(&mut self, value: u16){
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    /// # set_flag
    /// 
    /// Sets the flag to the value given, type `u8`
    /// 
    /// xxxx 0000 must be the given type
    /// 
    /// values:
    /// 
    /// 7654 3210
    /// 
    /// 7: Zero Flag (Did calculation result in 0)
    /// 6: Subtraction Flag (Was calculation a subtraction)
    /// 5: Half-Carry Flag (Did any byte from the lower nibble overflow to the upper nibble)
    /// 4: Carry Flag (Did the calculation overflow)
    pub fn set_zero(&mut self, value: bool){
        self.f.zero = value;
    }
    pub fn set_sub(&mut self, value: bool){
        self.f.sub = value;
    }
    pub fn set_half(&mut self, value: bool){
        self.f.half = value;
    }
    pub fn set_carry(&mut self, value: bool){
        self.f.carry = value;
    }
    
    pub fn get_zero(&self) -> bool{
        self.f.zero
    }
    pub fn get_sub(&self) -> bool{
        self.f.sub
    }
    pub fn get_half(&self) -> bool{
        self.f.half
    }
    pub fn get_carry(&self) -> bool{
        self.f.carry
    }

}