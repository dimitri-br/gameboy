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

const CPU_COMMANDS : [&str; 512] = [
    "NOP",
    "LD BC,d16",
    "LD (BC),A",
    "INC BC",
    "INC B",
    "DEC B",
    "LD B,d8",
    "RLCA",
    "LD (a16),SP",
    "ADD HL,BC",
    "LD A,(BC)",
    "DEC BC",
    "INC C",
    "DEC C",
    "LD C,d8",
    "RRCA",
    "STOP 0",
    "LD DE,d16",
    "LD (DE),A",
    "INC DE",
    "INC D",
    "DEC D",
    "LD D,d8",
    "RLA",
    "JR r8",
    "ADD HL,DE",
    "LD A,(DE)",
    "DEC DE",
    "INC E",
    "DEC E",
    "LD E,d8",
    "RRA",
    "JR NZ,r8",
    "LD HL,d16",
    "LD (HL+),A",
    "INC HL",
    "INC H",
    "DEC H",
    "LD H,d8",
    "DAA",
    "JR Z,r8",
    "ADD HL,HL",
    "LD A,(HL+)",
    "DEC HL",
    "INC L",
    "DEC L",
    "LD L,d8",
    "CPL",
    "JR NC,r8",
    "LD SP,d16",
    "LD (HL-),A",
    "INC SP",
    "INC (HL)",
    "DEC (HL)",
    "LD (HL),d8",
    "SCF",
    "JR C,r8",
    "ADD HL,SP",
    "LD A,(HL-)",
    "DEC SP",
    "INC A",
    "DEC A",
    "LD A,d8",
    "CCF",
    "LD B,B",
    "LD B,C",
    "LD B,D",
    "LD B,E",
    "LD B,H",
    "LD B,L",
    "LD B,(HL)",
    "LD B,A",
    "LD C,B",
    "LD C,C",
    "LD C,D",
    "LD C,E",
    "LD C,H",
    "LD C,L",
    "LD C,(HL)",
    "LD C,A",
    "LD D,B",
    "LD D,C",
    "LD D,D",
    "LD D,E",
    "LD D,H",
    "LD D,L",
    "LD D,(HL)",
    "LD D,A",
    "LD E,B",
    "LD E,C",
    "LD E,D",
    "LD E,E",
    "LD E,H",
    "LD E,L",
    "LD E,(HL)",
    "LD E,A",
    "LD H,B",
    "LD H,C",
    "LD H,D",
    "LD H,E",
    "LD H,H",
    "LD H,L",
    "LD H,(HL)",
    "LD H,A",
    "LD L,B",
    "LD L,C",
    "LD L,D",
    "LD L,E",
    "LD L,H",
    "LD L,L",
    "LD L,(HL)",
    "LD L,A",
    "LD (HL),B",
    "LD (HL),C",
    "LD (HL),D",
    "LD (HL),E",
    "LD (HL),H",
    "LD (HL),L",
    "HALT",
    "LD (HL),A",
    "LD A,B",
    "LD A,C",
    "LD A,D",
    "LD A,E",
    "LD A,H",
    "LD A,L",
    "LD A,(HL)",
    "LD A,A",
    "ADD A,B",
    "ADD A,C",
    "ADD A,D",
    "ADD A,E",
    "ADD A,H",
    "ADD A,L",
    "ADD A,(HL)",
    "ADD A,A",
    "ADC A,B",
    "ADC A,C",
    "ADC A,D",
    "ADC A,E",
    "ADC A,H",
    "ADC A,L",
    "ADC A,(HL)",
    "ADC A,A",
    "SUB B",
    "SUB C",
    "SUB D",
    "SUB E",
    "SUB H",
    "SUB L",
    "SUB (HL)",
    "SUB A",
    "SBC A,B",
    "SBC A,C",
    "SBC A,D",
    "SBC A,E",
    "SBC A,H",
    "SBC A,L",
    "SBC A,(HL)",
    "SBC A,A",
    "AND B",
    "AND C",
    "AND D",
    "AND E",
    "AND H",
    "AND L",
    "AND (HL)",
    "AND A",
    "XOR B",
    "XOR C",
    "XOR D",
    "XOR E",
    "XOR H",
    "XOR L",
    "XOR (HL)",
    "XOR A",
    "OR B",
    "OR C",
    "OR D",
    "OR E",
    "OR H",
    "OR L",
    "OR (HL)",
    "OR A",
    "CP B",
    "CP C",
    "CP D",
    "CP E",
    "CP H",
    "CP L",
    "CP (HL)",
    "CP A",
    "RET NZ",
    "POP BC",
    "JP NZ,a16",
    "JP a16",
    "CALL NZ,a16",
    "PUSH BC",
    "ADD A,d8",
    "RST 00H",
    "RET Z",
    "RET",
    "JP Z,a16",
    "PREFIX CB",
    "CALL Z,a16",
    "CALL a16",
    "ADC A,d8",
    "RST 08H",
    "RET NC",
    "POP DE",
    "JP NC,a16",
    "",
    "CALL NC,a16",
    "PUSH DE",
    "SUB d8",
    "RST 10H",
    "RET C",
    "RETI",
    "JP C,a16",
    "",
    "CALL C,a16",
    "",
    "SBC A,d8",
    "RST 18H",
    "LDH (a8),A",
    "POP HL",
    "LD (C),A",
    "",
    "",
    "PUSH HL",
    "AND d8",
    "RST 20H",
    "ADD SP,r8",
    "JP (HL)",
    "LD (a16),A",
    "",
    "",
    "",
    "XOR d8",
    "RST 28H",
    "LDH A,(a8)",
    "POP AF",
    "LD A,(C)",
    "DI",
    "",
    "PUSH AF",
    "OR d8",
    "RST 30H",
    "LD HL,SP+r8",
    "LD SP,HL",
    "LD A,(a16)",
    "EI",
    "",
    "",
    "CP d8",
    "RST 38H",
    "RLC B",
    "RLC C",
    "RLC D",
    "RLC E",
    "RLC H",
    "RLC L",
    "RLC (HL)",
    "RLC A",
    "RRC B",
    "RRC C",
    "RRC D",
    "RRC E",
    "RRC H",
    "RRC L",
    "RRC (HL)",
    "RRC A",
    "RL B",
    "RL C",
    "RL D",
    "RL E",
    "RL H",
    "RL L",
    "RL (HL)",
    "RL A",
    "RR B",
    "RR C",
    "RR D",
    "RR E",
    "RR H",
    "RR L",
    "RR (HL)",
    "RR A",
    "SLA B",
    "SLA C",
    "SLA D",
    "SLA E",
    "SLA H",
    "SLA L",
    "SLA (HL)",
    "SLA A",
    "SRA B",
    "SRA C",
    "SRA D",
    "SRA E",
    "SRA H",
    "SRA L",
    "SRA (HL)",
    "SRA A",
    "SWAP B",
    "SWAP C",
    "SWAP D",
    "SWAP E",
    "SWAP H",
    "SWAP L",
    "SWAP (HL)",
    "SWAP A",
    "SRL B",
    "SRL C",
    "SRL D",
    "SRL E",
    "SRL H",
    "SRL L",
    "SRL (HL)",
    "SRL A",
    "BIT 0,B",
    "BIT 0,C",
    "BIT 0,D",
    "BIT 0,E",
    "BIT 0,H",
    "BIT 0,L",
    "BIT 0,(HL)",
    "BIT 0,A",
    "BIT 1,B",
    "BIT 1,C",
    "BIT 1,D",
    "BIT 1,E",
    "BIT 1,H",
    "BIT 1,L",
    "BIT 1,(HL)",
    "BIT 1,A",
    "BIT 2,B",
    "BIT 2,C",
    "BIT 2,D",
    "BIT 2,E",
    "BIT 2,H",
    "BIT 2,L",
    "BIT 2,(HL)",
    "BIT 2,A",
    "BIT 3,B",
    "BIT 3,C",
    "BIT 3,D",
    "BIT 3,E",
    "BIT 3,H",
    "BIT 3,L",
    "BIT 3,(HL)",
    "BIT 3,A",
    "BIT 4,B",
    "BIT 4,C",
    "BIT 4,D",
    "BIT 4,E",
    "BIT 4,H",
    "BIT 4,L",
    "BIT 4,(HL)",
    "BIT 4,A",
    "BIT 5,B",
    "BIT 5,C",
    "BIT 5,D",
    "BIT 5,E",
    "BIT 5,H",
    "BIT 5,L",
    "BIT 5,(HL)",
    "BIT 5,A",
    "BIT 6,B",
    "BIT 6,C",
    "BIT 6,D",
    "BIT 6,E",
    "BIT 6,H",
    "BIT 6,L",
    "BIT 6,(HL)",
    "BIT 6,A",
    "BIT 7,B",
    "BIT 7,C",
    "BIT 7,D",
    "BIT 7,E",
    "BIT 7,H",
    "BIT 7,L",
    "BIT 7,(HL)",
    "BIT 7,A",
    "RES 0,B",
    "RES 0,C",
    "RES 0,D",
    "RES 0,E",
    "RES 0,H",
    "RES 0,L",
    "RES 0,(HL)",
    "RES 0,A",
    "RES 1,B",
    "RES 1,C",
    "RES 1,D",
    "RES 1,E",
    "RES 1,H",
    "RES 1,L",
    "RES 1,(HL)",
    "RES 1,A",
    "RES 2,B",
    "RES 2,C",
    "RES 2,D",
    "RES 2,E",
    "RES 2,H",
    "RES 2,L",
    "RES 2,(HL)",
    "RES 2,A",
    "RES 3,B",
    "RES 3,C",
    "RES 3,D",
    "RES 3,E",
    "RES 3,H",
    "RES 3,L",
    "RES 3,(HL)",
    "RES 3,A",
    "RES 4,B",
    "RES 4,C",
    "RES 4,D",
    "RES 4,E",
    "RES 4,H",
    "RES 4,L",
    "RES 4,(HL)",
    "RES 4,A",
    "RES 5,B",
    "RES 5,C",
    "RES 5,D",
    "RES 5,E",
    "RES 5,H",
    "RES 5,L",
    "RES 5,(HL)",
    "RES 5,A",
    "RES 6,B",
    "RES 6,C",
    "RES 6,D",
    "RES 6,E",
    "RES 6,H",
    "RES 6,L",
    "RES 6,(HL)",
    "RES 6,A",
    "RES 7,B",
    "RES 7,C",
    "RES 7,D",
    "RES 7,E",
    "RES 7,H",
    "RES 7,L",
    "RES 7,(HL)",
    "RES 7,A",
    "SET 0,B",
    "SET 0,C",
    "SET 0,D",
    "SET 0,E",
    "SET 0,H",
    "SET 0,L",
    "SET 0,(HL)",
    "SET 0,A",
    "SET 1,B",
    "SET 1,C",
    "SET 1,D",
    "SET 1,E",
    "SET 1,H",
    "SET 1,L",
    "SET 1,(HL)",
    "SET 1,A",
    "SET 2,B",
    "SET 2,C",
    "SET 2,D",
    "SET 2,E",
    "SET 2,H",
    "SET 2,L",
    "SET 2,(HL)",
    "SET 2,A",
    "SET 3,B",
    "SET 3,C",
    "SET 3,D",
    "SET 3,E",
    "SET 3,H",
    "SET 3,L",
    "SET 3,(HL)",
    "SET 3,A",
    "SET 4,B",
    "SET 4,C",
    "SET 4,D",
    "SET 4,E",
    "SET 4,H",
    "SET 4,L",
    "SET 4,(HL)",
    "SET 4,A",
    "SET 5,B",
    "SET 5,C",
    "SET 5,D",
    "SET 5,E",
    "SET 5,H",
    "SET 5,L",
    "SET 5,(HL)",
    "SET 5,A",
    "SET 6,B",
    "SET 6,C",
    "SET 6,D",
    "SET 6,E",
    "SET 6,H",
    "SET 6,L",
    "SET 6,(HL)",
    "SET 6,A",
    "SET 7,B",
    "SET 7,C",
    "SET 7,D",
    "SET 7,E",
    "SET 7,H",
    "SET 7,L",
    "SET 7,(HL)",
    "SET 7,A",
];

pub struct CPU{
    pub memory: Memory,
    pub registers: Registers,
    pub ime: bool,
    pub delay: u32,
    pub pause: bool,
    pub halted: bool,
    pub debug: bool,
    pub ime_delay: bool,
    pub trace: Vec::<String>,
}


//new
impl CPU {
    pub fn new() -> Self{
        CPU { 
            memory: Memory::new(),
            registers: Registers::new(),
            ime: true,
            delay: 4,
            pause: false,
            halted: false,
            debug: false,
            ime_delay: false,
            trace: Vec::<String>::new(),


        }
    }
    pub fn init(&mut self){
        self.registers.set_af(0x01B0);
        self.registers.set_bc(0x0013);
        self.registers.set_de(0x00D8);
        self.registers.set_hl(0x014D);
        self.registers.sp = 0xFFFE;
        self.memory.set_initial();
    }
    pub fn load_rom(&mut self){
        let mut rom = ROM::new(String::from("./roms/boot.bin"));
        rom.load();
        let mut index = 0x0;
        for line in rom.content.iter(){
            self.memory.bios[index] = *line;
            index += 1;
        }

        let mut rom = ROM::new(String::from("./roms/red.gb")); //TODO - :)
        rom.load();
        let mut index = 0x0;
        for line in rom.content.iter(){
            self.memory.rom.push(*line);
            index += 1;
        }
        println!("• Length of ROM: {} -> {}", index, self.memory.rom.len());

        self.memory.carttype = self.memory.rom[0x0147];
        println!("• Cart Type: {:#x?}",self.memory.carttype);

    }
}

//opcodes
impl CPU {
    pub fn step(&mut self){
        let max_update = 69905;
        self.delay = 0;
        self.trace = Vec::<String>::new();
        while self.delay < max_update{
            

            let mut delay = 0x0;
            self.check_interrupts();
            self.memory.pc = self.registers.pc;


            if self.ime_delay{
                
                self.ime_delay = false;
                self.ime = true;
            }
            if !self.halted{
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

                /*let if_v = self.memory.timer.do_cycle(self.delay);
                self.memory.interrupt_flags |= if_v;*/
                

                let f = self.registers.get_f();
                let mut trace = String::new();
                if opcode == 0xCB{
                    let cb = self.memory.rb(self.registers.pc + 1);
                    trace = format!("A: {:x?} F: {:x?} B: {:x?} C: {:x?} D: {:x?} E: {:x?} H: {:x?} L: {:x?} SP: {:x?} PC: 00:{:x?} | Opcode: {:x?} : {:x?} -- {:x?}|{}\n", self.registers.a, f, self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l, self.registers.sp, self.registers.pc, opcode, cb, v, CPU_COMMANDS[opcode as usize]);
                }else{
                    trace = format!("A: {:x?} F: {:x?} B: {:x?} C: {:x?} D: {:x?} E: {:x?} H: {:x?} L: {:x?} SP: {:x?} PC: 00:{:x?} | Opcode: {:x?} -- {:x?}|{}\n", self.registers.a, f, self.registers.b, self.registers.c, self.registers.d, self.registers.e, self.registers.h, self.registers.l, self.registers.sp, self.registers.pc, opcode, v, CPU_COMMANDS[opcode as usize]);
                }
                //println!("{}",trace);
                //self.trace.push(trace);
                if self.registers.pc == 0x100{
                    self.memory.in_bios = false;
                }
                delay = (self.execute(opcode, v) as u32) / 4; //Make it M

                
                
            }else{   
                //If halted           
                delay = 1;
                
            }
           
            
                    
            self.delay += delay;

            self.memory.gpu.do_cycle(delay);

                

            
            self.memory.interrupt_flags |= self.memory.timer.do_cycle(delay * 4);

            self.memory.interrupt_flags |= self.memory.gpu.interrupt;
            self.memory.gpu.interrupt = 0;
        }
        
        
    }


    fn check_interrupts(&mut self){
        let IF = self.memory.rb(0xFF0F);
        let IE = self.memory.rb(0xFFFF);
        
        
        let potential_interrupts = IF & IE;
        if potential_interrupts == 0{
            return
        }else{
            
            self.halted = false;
        }
       
        if !self.ime{
            return
        }
        self.halted = false;
        for b in 0..5{
            if (potential_interrupts & (1 << b)) == 0{
                continue
            }

            //This does not get called
            self.memory.wb(0xFF0F, IF & !(1 << b));

            self.ime = false;
            self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
            self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
            self.registers.sp = self.registers.sp.wrapping_sub(2);
            match b{
                0 => { self.registers.pc = 0x40;},
                1 => { self.registers.pc = 0x48;},
                2 => { self.registers.pc = 0x50;},
                3 => { self.registers.pc = 0x58;},
                4 => { self.registers.pc = 0x60;},
                _ => { panic!("Unknown IF!");}
            }
            return
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
                0x2 => {
                    self.memory.wb(self.registers.get_bc(), self.registers.a);
                    self.registers.pc += 1;
                    8
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
                0x7 => { //RLCA
                    let c = self.registers.a & 0x80 == 0x80;
                    let new_val = self.registers.a.wrapping_shl(1).wrapping_add(if c { 1 }else{ 0 });

                    self.registers.set_zero(false);
                    self.registers.set_sub(false);
                    self.registers.set_half(false);
                    self.registers.set_carry(self.registers.a & 0x80 != 0);


                    self.registers.a = new_val;

                    self.registers.pc += 1;
                    4
                }
                0x8 => {
                    
                    self.memory.wb(v as u16, (self.registers.sp & 0xFF) as u8);
                    self.memory.wb((v + 1)as u16, (self.registers.sp >> 8) as u8);
                    self.registers.pc += 3;
                    20
                }
                0x9 => {
                    self.add(Target::HL, self.registers.get_bc() as usize);
                    self.registers.pc += 1;
                    8
                }
                0xA => {
                    let val = self.memory.rb(self.registers.get_bc()) as usize;
                    self.ld(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0xB => {
                    self.dec(Target::BC);
                    self.registers.pc += 1;
                    8
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
                0xF => { //RRCA
                    let c = self.registers.a & 0x1 == 0x1;
                    let new_val = self.registers.a.wrapping_shr(1).wrapping_add(if c { 0x80 }else{ 0 });

                    self.registers.set_zero(false);
                    self.registers.set_sub(false);
                    self.registers.set_half(false);
                    self.registers.set_carry(self.registers.a & 0x1 != 0);

                    self.registers.pc += 1;
                    self.registers.a = new_val;
                    4
                }
                0x10 => {
                    self.registers.pc += 2;
                    4
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
                0x1B => {
                    self.dec(Target::DE);
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
                    let mut a = self.registers.a;
                    let mut adjust = if self.registers.get_carry() { 0x60 }else{ 0 };
                    if self.registers.get_half() { adjust |= 0x6; };

                    if !self.registers.get_sub(){
                        if a & 0x0F > 0x09 { adjust |= 0x06; };
                        if a > 0x99 { adjust |= 0x60; };
                        a = a.wrapping_add(adjust);
                    } else {
                        a = a.wrapping_sub(adjust);
                    }

                    self.registers.set_zero(a == 0);
                    self.registers.set_half(false);
                    self.registers.set_carry(adjust >= 0x60);


                    self.registers.a = a;
                    self.registers.pc += 1;
                    
                    
                    4 //timer
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
                0x2B => {
                    self.dec(Target::HL);
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
                    let val = self.memory.rb(self.registers.get_hl());
                    let (new_value, _did_overflow) = val.overflowing_add(1 as u8);
                    self.registers.set_zero(new_value == 0);
                    self.registers.set_sub(false);
                    self.registers.set_half((val & 0xF) + ((1 as u8) & 0xF) > 0xF);
               
                    self.memory.wb(self.registers.get_hl(), new_value);
                    self.registers.pc += 1;
                    12
                }
                0x35 => {
                    let val = self.memory.rb(self.registers.get_hl());
                    let new_val = val.wrapping_sub(1);
                    self.registers.set_zero(new_val == 0);
                    self.registers.set_sub(true);
                    self.registers.set_half(((val as i16) & 0xF) - ((1 as i16) & 0xF) < 0x0);
                    self.memory.wb(self.registers.get_hl(), new_val);
                    self.registers.pc += 1;
                    12
                }
                0x36 => {
                    self.memory.wb(self.registers.get_hl(), v as u8);
                    self.registers.pc += 2;
                    12
                }
                0x37 => {
                    self.scf();
                    self.registers.pc += 1;
                    4
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
                0x3A => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.ld(Target::A, val);
                    self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
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
                0x3F => {
                    self.ccf();
                    self.registers.pc += 1;
                    4
                }
                0x40 => {
                    self.ld(Target::B, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x41 => {
                    self.ld(Target::B, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x42 => {
                    self.ld(Target::B, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x43 => {
                    self.ld(Target::B, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x44 => {
                    self.ld(Target::B, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x45 => {
                    self.ld(Target::B, self.registers.l as usize);
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
                0x48 => {
                    self.ld(Target::C, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x49 => {
                    self.ld(Target::C, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x4A => {
                    self.ld(Target::C, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x4B => {
                    self.ld(Target::C, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x4C => {
                    self.ld(Target::C, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x4D => {
                    self.ld(Target::C, self.registers.l as usize);
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
                0x50 => {
                    self.ld(Target::D, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x51 => {
                    self.ld(Target::D, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x52 => {
                    self.ld(Target::D, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x53 => {
                    self.ld(Target::D, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x54 => {
                    self.ld(Target::D, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x55 => {
                    self.ld(Target::D, self.registers.l as usize);
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
                0x59 => {
                    self.ld(Target::E, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x5A => {
                    self.ld(Target::E, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x5B => {
                    self.ld(Target::E, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x5C => {
                    self.ld(Target::E, self.registers.h as usize);
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
                0x60 => {
                    self.ld(Target::H, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x61 => {
                    self.ld(Target::H, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x62 => {
                    self.ld(Target::H, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x63 => {
                    self.ld(Target::H, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x64 => {
                    self.ld(Target::H, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x65 => {
                    self.ld(Target::H, self.registers.l as usize);
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
                0x68 => {
                    self.ld(Target::L, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x69 => {
                    self.ld(Target::L, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x6A => {
                    self.ld(Target::L, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x6B => {
                    self.ld(Target::L, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x6C => {
                    self.ld(Target::L, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x6D => {
                    self.ld(Target::L, self.registers.l as usize);
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
                0x74 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.h);
                    self.registers.pc += 1;
                    8
                }
                0x75 => {
                    self.memory.wb(self.registers.get_hl(), self.registers.l);
                    self.registers.pc += 1;
                    8
                }
                0x76 => {//HALT
                    self.halted = true;
                    self.registers.pc += 1;
                    4
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
                0x7F => {
                    self.ld(Target::A, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x80 => {
                    self.add(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x81 => {
                    self.add(Target::A, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x82 => {
                    self.add(Target::A, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x83 => {
                    self.add(Target::A, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x84 => {
                    self.add(Target::A, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x85 => {
                    self.add(Target::A, self.registers.l as usize);
                    self.registers.pc += 1;
                    4
                }
                0x86 => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.add(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0x87 => {
                    self.add(Target::A, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
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
                0x8A => {
                    self.adc(Target::A, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x8B => {
                    self.adc(Target::A, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x8C => {
                    self.adc(Target::A, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x8D => {
                    self.adc(Target::A, self.registers.l as usize);
                    self.registers.pc += 1;
                    4
                }
                0x8E => {
                    let val = self.memory.rb(self.registers.get_hl());
                    self.adc(Target::A, val as usize);
                    self.registers.pc += 1;
                    8
                }
                0x8F => {
                    self.adc(Target::A, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x90 => {
                    self.sub(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;               
                    4
                }
                0x91 => {
                    self.sub(Target::A, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x92 => {
                    self.sub(Target::A, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x93 => {
                    self.sub(Target::A, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x94 => {
                    self.sub(Target::A, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x95 => {
                    self.sub(Target::A, self.registers.l as usize);
                    self.registers.pc += 1;
                    4
                }
                0x96 => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.sub(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0x97 => {
                    self.sub(Target::A, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0x98 => {
                    self.sbc(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0x99 => {
                    self.sbc(Target::A, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0x9A => {
                    self.sbc(Target::A, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0x9B => {
                    self.sbc(Target::A, self.registers.e as usize);
                    self.registers.pc += 1;
                    4
                }
                0x9C => {
                    self.sbc(Target::A, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0x9D => {
                    self.sbc(Target::A, self.registers.l as usize);
                    self.registers.pc += 1;
                    4
                }
                0x9E => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.sbc(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0x9F => {
                    self.sbc(Target::A, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0xA0 => {
                    self.and(self.registers.b);
                    self.registers.pc += 1;
                    4
                }
                0xA1 => {
                    self.and(self.registers.c);
                    self.registers.pc += 1;
                    4
                }
                0xA2 => {
                    self.and(self.registers.d);
                    self.registers.pc += 1;
                    4
                }
                0xA3 => {
                    self.and(self.registers.e);
                    self.registers.pc += 1;
                    4
                }
                0xA4 => {
                    self.and(self.registers.h);
                    self.registers.pc += 1;
                    4
                }
                0xA5 => {
                    self.and(self.registers.l);
                    self.registers.pc += 1;
                    4
                }
                0xA6 => {
                    let val = self.memory.rb(self.registers.get_hl());
                    self.and(val);
                    self.registers.pc += 1;
                    8
                }
                0xA7 => {
                    self.and(self.registers.a);
                    self.registers.pc += 1;
                    4
                }
                0xA8 => {
                    self.xor(self.registers.b);
                    self.registers.pc += 1;
                    4
                }
                0xA9 => {
                    self.xor(self.registers.c);
                    self.registers.pc += 1;
                    4
                }
                0xAA => {
                    self.xor(self.registers.d);
                    self.registers.pc += 1;
                    4
                }
                0xAB => {
                    self.xor(self.registers.e);
                    self.registers.pc += 1;
                    4
                }
                0xAC => {
                    self.xor(self.registers.h);
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
                0xB2 => {
                    self.or(self.registers.d);
                    self.registers.pc += 1;
                    4
                }
                0xB3 => {
                    self.or(self.registers.e);
                    self.registers.pc += 1;
                    4
                }
                0xB4 => {
                    self.or(self.registers.h);
                    self.registers.pc += 1;
                    4
                }
                0xB5 => {
                    self.or(self.registers.l);
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
                0xB8 => {
                    self.cp(Target::A, self.registers.b as usize);
                    self.registers.pc += 1;
                    4
                }
                0xB9 => {
                    self.cp(Target::A, self.registers.c as usize);
                    self.registers.pc += 1;
                    4
                }
                0xBA => {
                    self.cp(Target::A, self.registers.d as usize);
                    self.registers.pc += 1;
                    4
                }
                0xBB => {


                    self.cp(Target::A, self.registers.e as usize);


                    self.registers.pc += 1;
                    4
                }
                0xBC => {
                    self.cp(Target::A, self.registers.h as usize);
                    self.registers.pc += 1;
                    4
                }
                0xBD => {
                    self.cp(Target::A, self.registers.l as usize);
                    self.registers.pc += 1;
                    4
                }
                0xBE => {
                    let val = self.memory.rb(self.registers.get_hl()) as usize;
                    self.cp(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0xBF => {
                    self.cp(Target::A, self.registers.a as usize);
                    self.registers.pc += 1;
                    4
                }
                0xC0 => {
                    if self.registers.get_zero() == false{
                        self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                        self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                        self.registers.sp = self.registers.sp.wrapping_add(2);
                        20
                    }else{
                        self.registers.pc += 1;
                        8
                    }
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
                0xC7 => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 0;
                    16
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
                0xCA => {
                    if self.registers.get_zero(){
                        self.registers.pc = v as u16;
                        16
                    }else{
                        self.registers.pc += 3;
                        12
                    }
                }
                0xCB => {
                    self.registers.pc += 1;
                    let val = self.memory.rb(self.registers.pc);
                    let delay = self.execute_cb(val, v);
                    self.registers.pc += 1;
                    delay
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
                0xCF => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 8;
                    16
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
                0xD2 => {
                    if self.registers.get_carry() == false{
                        self.registers.pc = v as u16;
                        16
                    }else{
                        self.registers.pc += 3;
                        12
                    }
                }
                0xD4 => {
                    self.registers.pc += 3;
                    if self.registers.get_carry() == false{
                        self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                        self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                        self.registers.sp = self.registers.sp.wrapping_sub(2);
                        self.registers.pc = v as u16;
                        24
                    }else{
                        12
                    }
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
                0xD7 => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 16;
                    16
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
                    self.ime = true;
                    self.registers.pc = (self.memory.rb(self.registers.sp.wrapping_add(1)) as u16) << 8;
                    self.registers.pc |= self.memory.rb(self.registers.sp) as u16;
                    self.registers.sp = self.registers.sp.wrapping_add(2);
                    16
                }
                0xDA => {
                    if self.registers.get_carry(){
                        self.registers.pc = v as u16;
                        16
                    }else{
                        self.registers.pc += 3;
                        12
                    }

                }
                0xDC => {
                    self.registers.pc += 3;
                    if self.registers.get_carry(){
                        self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                        self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                        self.registers.sp = self.registers.sp.wrapping_sub(2);
                        self.registers.pc = v as u16;
                        24
                    }else{
                        12
                    }
                }
                0xDD => {
                    self.registers.pc += 1;
                    4
                }
                0xDE => {
                    self.sbc(Target::A, v);
                    self.registers.pc += 2;
                    8
                }
                0xDF => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 24;
                    16
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
                0xE3 => {
                    self.registers.pc += 1;
                    4
                }
                0xE4 => {
                    self.registers.pc += 1;
                    4
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
                0xE7 => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 32;
                    16
                }
                0xE8 => {
                    let new_val = v as i8;
                    self.add(Target::SP, new_val as usize);
                    self.registers.pc += 2;
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
                0xED => {
                    self.registers.pc += 1;
                    4
                }
                0xEE => {
                    self.xor(v as u8);
                    self.registers.pc += 2;
                    8
                }
                0xEF => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 40;
                    16
                }
                0xF0 => {
                    let mem_loc = 0xFF00 + (v as u8) as u16;
                    let val = self.memory.rb(mem_loc);
                    if v == 0xE3{
                        //println!("Read {:#x?} from {:#x?}", val, 0xFF00 + v);
                    }
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
                0xF2 => {
                    let val = self.memory.rb(0xFF00 + (self.registers.c as u16)) as usize;
                    self.ld(Target::A, val);
                    self.registers.pc += 1;
                    8
                }
                0xF3 => {
                    self.ime_delay = false;
                    self.ime = false;
                    self.registers.pc += 1;
                    4
                }
                0xF4 => {
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
                0xF6 => {
                    self.or(v as u8);
                    self.registers.pc += 2;
                    8
                }
                0xF7 => {
                    self.registers.pc += 1;
                    self.memory.wb(self.registers.sp.wrapping_sub(1), (self.registers.pc >> 8) as u8);
                    self.memory.wb(self.registers.sp.wrapping_sub(2), (self.registers.pc & 0xFF) as u8);
                    self.registers.sp = self.registers.sp.wrapping_sub(2);
                    self.registers.pc = 48;
                    16
                }
                0xF8 => {
                    let sp = self.registers.sp as i16;
                    let v = (v as i8) as i16;
                    let new_value = sp.wrapping_add(v);

                    

                    self.registers.set_zero(false);
                    self.registers.set_sub(false);
                    self.registers.set_carry((sp & 0xFF) + (v & 0xFF) > 0xFF);
                    self.registers.set_half((sp & 0xF) + (v & 0xF) > 0xF);

                    self.ld(Target::HL, new_value as usize);
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
                0xFB => { //EI
                    self.ime_delay = true;
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
            0x0 => {
                self.rlc(Target::B);
                8
            }
            0x1 => {
                self.rlc(Target::C);
                8
            }
            0x2 => {
                self.rlc(Target::D);
                8
            }
            0x3 => {
                self.rlc(Target::E);
                8
            }
            0x4 => {
                self.rlc(Target::H);
                8
            }
            0x5 => {
                self.rlc(Target::L);
                8
                
            }
            0x6 => {
                self.rlc(Target::HL);
                16
            }
            0x7 => {
                self.rlc(Target::A);
                8
            }
            0x8 => {
                self.rrc(Target::B);
                8
            }
            0x9 => {
                self.rrc(Target::C);
                8
            }
            0xA => {
                self.rrc(Target::D);
                8
            }
            0xB => {
                self.rrc(Target::E);
                8
            }
            0xC => {
                self.rrc(Target::H);
                8
            }
            0xD => {
                self.rrc(Target::L);
                8
            }
            0xE => {
                self.rrc(Target::HL);
                16
            }
            0xF => {
                self.rrc(Target::A);
                8
            }
            0x10 => {
                self.rl(Target::B);
                8
            }
            0x11 => {
                self.rl(Target::C);
                8
            }
            0x12 => {
                self.rl(Target::D);
                8
            }
            0x13 => {
                self.rl(Target::E);
                8
            }
            0x14 => {
                self.rl(Target::H);
                8
            }
            0x15 => {
                self.rl(Target::L);
                8
            }
            0x16 => {
                self.rl(Target::HL);
                16
            }
            0x17 => {
                self.rl(Target::A);
                8
            }
            0x18 => {
                self.rr(Target::B);
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
            0x1C => {
                self.rr(Target::H);
                8
            }
            0x1D => {
                self.rr(Target::L);
                8
            }
            0x1E => {
                self.rr(Target::HL);
                16
            }
            0x1F => {
                self.rr(Target::A);
                8
            }
            0x20 => {
                self.sla(Target::B);
                8
            }
            0x21 => {
                self.sla(Target::C);
                8
            }
            0x22 => {
                self.sla(Target::D);
                8
            }
            0x23 => {
                self.sla(Target::E);
                8
            }
            0x24 => {
                self.sla(Target::H);
                8
            }
            0x25 => {
                self.sla(Target::L);
                8
            }
            0x26 => {
                self.sla(Target::HL);
                16
            }
            0x27 => {
                self.sla(Target::A);
                8
            }
            0x28 => {
                self.sra(Target::B);
                8
            }
            0x29 => {
                self.sra(Target::C);
                8
            }
            0x2A => {
                self.sra(Target::D);
                8
            }
            0x2B => {
                self.sra(Target::E);
                8
            }
            0x2C => {
                self.sra(Target::H);
                8
            }
            0x2D => {
                self.sra(Target::L);
                8
            }
            0x2E => {
                self.sra(Target::HL);
                16
            }
            0x2F => {
                self.sra(Target::A);
                8
            }
            0x30 => {
                self.swap(Target::B);
                8
            }
            0x31 => {
                self.swap(Target::C);
                8
            }
            0x32=> {
                self.swap(Target::D);
                8
            }
            0x33 => {
                self.swap(Target::E);
                8
            }
            0x34 => {
                self.swap(Target::H);
                8
            }
            0x35 => {
                self.swap(Target::L);
                8
            }
            0x36 => {
                self.swap(Target::HL);
                16
            }
            0x37 => {
                self.swap(Target::A);
                8
            }
            0x38 => {
                self.srl(Target::B);
                8
            }
            0x39 => {
                self.srl(Target::C);
                8
            }
            0x3A => {
                self.srl(Target::D);
                8
            }
            0x3B => {
                self.srl(Target::E);
                8
            }
            0x3C => {
                self.srl(Target::H);
                8
            }
            0x3D => {
                self.srl(Target::L);
                8
            }
            0x3E => {
                self.srl(Target::HL);
                16
            }
            0x3F => {
                self.srl(Target::A);
                8
            }
            0x40 => {
                self.bit(Target::B, 0);
                8
            }
            0x41 => {
                self.bit(Target::C, 0);
                8
            }
            0x42 => {
                self.bit(Target::D, 0);
                8
            }
            0x43 => {
                self.bit(Target::E, 0);
                8
            }
            0x44 => {
                self.bit(Target::H, 0);
                8
            }
            0x45 => {
                self.bit(Target::L, 0);
                8
            }
            0x46 => {
                self.bit(Target::HL, 0);
                12
            }
            0x47 => {
                self.bit(Target::A, 0);
                8
            }
            0x48 => {
                self.bit(Target::B, 1);
                8
            }
            0x49 => {
                self.bit(Target::C, 1);
                8
            }
            0x4A => {
                self.bit(Target::D, 1);
                8
            }
            0x4B => {
                self.bit(Target::E, 1);
                8
            }
            0x4C => {
                self.bit(Target::H, 1);
                8
            }
            0x4D => {
                self.bit(Target::L, 1);
                8
            }
            0x4E => {
                self.bit(Target::HL, 1);
                12
            }
            0x4F => {
                self.bit(Target::A, 1);
                8
            }
            0x50 => {
                self.bit(Target::B, 2);
                8
            }
            0x51 => {
                self.bit(Target::C, 2);
                8
            }
            0x52 => {
                self.bit(Target::D, 2);
                8
            }
            0x53 => {
                self.bit(Target::E, 2);
                8
            }
            0x54 => {
                self.bit(Target::H, 2);
                8
            }
            0x55 => {
                self.bit(Target::L, 2);
                8
            }
            0x56 => {
                self.bit(Target::HL, 2);
                12
            }
            0x57 => {
                self.bit(Target::A, 2);
                8
            }
            0x58 => {
                self.bit(Target::B, 3);
                8
            }
            0x59 => {
                self.bit(Target::C, 3);
                8
            }
            0x5A => {
                self.bit(Target::D, 3);
                8
            }
            0x5B => {
                self.bit(Target::E, 3);
                8
            }
            0x5C => {
                self.bit(Target::H, 3);
                8
            }
            0x5D => {
                self.bit(Target::L, 3);
                8
            }
            0x5E => {
                self.bit(Target::HL, 3);
                12
            }
            0x5F => {
                self.bit(Target::A, 3);
                8
            }
            0x60 => {
                self.bit(Target::B, 4);
                8
            }
            0x61 => {
                self.bit(Target::C, 4);
                8
            }
            0x62 => {
                self.bit(Target::D, 4);
                8
            }
            0x63 => {
                self.bit(Target::E, 4);
                8
            }
            0x64 => {
                self.bit(Target::H, 4);
                8
            }
            0x65 => {
                self.bit(Target::L, 4);
                8
            }
            0x66 => {
                self.bit(Target::HL, 4);
                12
            }
            0x67 => {
                self.bit(Target::A, 4);
                8
            }
            0x68 => {
                self.bit(Target::B, 5);
                8
            }
            0x69 => {
                self.bit(Target::C, 5);
                8
            }
            0x6A => {
                self.bit(Target::D, 5);
                8
            }
            0x6B => {
                self.bit(Target::E, 5);
                8
            }
            0x6C => {
                self.bit(Target::H, 5);
                8
            }
            0x6D => {
                self.bit(Target::L, 5);
                8
            }
            0x6E => {
                self.bit(Target::HL, 5);
                12
            }
            0x6F => {
                self.bit(Target::A, 5);
                8
            }
            0x70 => {
                self.bit(Target::B, 6);
                8
            }
            0x71 => {
                self.bit(Target::C, 6);
                8
            }
            0x72 => {
                self.bit(Target::D, 6);
                8
            }
            0x73 => {
                self.bit(Target::E, 6);
                8
            }
            0x74 => {
                self.bit(Target::H, 6);
                8
            }
            0x75 => {
                self.bit(Target::L, 6);
                8
            }
            0x76 => {
                self.bit(Target::HL, 6);
                12
            }
            0x77 => {
                self.bit(Target::A, 6);
                8
            }
            0x78 => {
                self.bit(Target::B, 7);
                8
            }
            0x79 => {
                self.bit(Target::C, 7);
                8
            }
            0x7A => {
                self.bit(Target::D, 7);
                8
            }
            0x7B => {
                self.bit(Target::E, 7);
                8
            }

            0x7C => {
                self.bit(Target::H, 7);
                8
            }
            0x7D => {
                self.bit(Target::L, 7);
                8
            }
            0x7E => {
                self.bit(Target::HL, 7);
                12
            }
            0x7F => {
                self.bit(Target::A, 7);
                8
            }
            0x80 => {
                self.reset(Target::B, 0);
                8
            }
            0x81 => {
                self.reset(Target::C, 0);
                8
            }
            0x82 => {
                self.reset(Target::D, 0);
                8
            }
            0x83 => {
                self.reset(Target::E, 0);
                8
            }
            0x84 => {
                self.reset(Target::H, 0);
                8
            }
            0x85 => {
                self.reset(Target::L, 0);
                8
            }
            0x86 => {
                self.reset(Target::HL, 0);
                16
            }
            0x87 => {
                self.reset(Target::A, 0);
                8
            }
            0x88 => {
                self.reset(Target::B, 1);
                8
            }
            0x89 => {
                self.reset(Target::C, 1);
                8
            }
            0x8A => {
                self.reset(Target::D, 1);
                8
            }
            0x8B => {
                self.reset(Target::E, 1);
                8
            }
            0x8C => {
                self.reset(Target::H, 1);
                8
            }
            0x8D => {
                self.reset(Target::L, 1);
                8
            }
            0x8E => {
                self.reset(Target::HL, 1);
                16
            }
            0x8F => {
                self.reset(Target::A, 1);
                8
            }
            0x90 => {
                self.reset(Target::B, 2);
                8
            }
            0x91 => {
                self.reset(Target::C, 2);
                8
            }
            0x92 => {
                self.reset(Target::D, 2);
                8
            }
            0x93 => {
                self.reset(Target::E, 2);
                8
            }
            0x94 => {
                self.reset(Target::H, 2);
                8
            }
            0x95 => {
                self.reset(Target::L, 2);
                8
            }
            0x96 => {
                self.reset(Target::HL, 2);
                16
            }
            0x97 => {
                self.reset(Target::A, 2);
                8
            }
            0x98 => {
                self.reset(Target::B, 3);
                8
            }
            0x99 => {
                self.reset(Target::C, 3);
                8
            }
            0x9A => {
                self.reset(Target::D, 3);
                8
            }
            0x9B => {
                self.reset(Target::E, 3);
                8
            }
            0x9C => {
                self.reset(Target::H, 3);
                8
            }
            0x9D => {
                self.reset(Target::L, 3);
                8
            }
            0x9E => {
                self.reset(Target::HL, 3);
                16
            }
            0x9F => {
                self.reset(Target::A, 3);
                8
            }
            0xA0 => {
                self.reset(Target::B, 4);
                8
            }
            0xA1 => {
                self.reset(Target::C, 4);
                8
            }
            0xA2 => {
                self.reset(Target::D, 4);
                8
            }
            0xA3 => {
                self.reset(Target::E, 4);
                8
            }
            0xA4 => {
                self.reset(Target::H, 4);
                8
            }
            0xA5 => {
                self.reset(Target::L, 4);
                8
            }
            0xA6 => {
                self.reset(Target::HL, 4);
                16
            }
            0xA7 => {
                self.reset(Target::A, 4);
                8
            }
            0xA8 => {
                self.reset(Target::B, 5);
                8
            }
            0xA9 => {
                self.reset(Target::C, 5);
                8
            }
            0xAA => {
                self.reset(Target::D, 5);
                8
            }
            0xAB => {
                self.reset(Target::E, 5);
                8
            }
            0xAC => {
                self.reset(Target::H, 5);
                8
            }
            0xAD => {
                self.reset(Target::L, 5);
                8
            }
            0xAE => {
                self.reset(Target::HL, 5);
                16
            }
            0xAF => {
                self.reset(Target::A, 5);
                8
            }
            0xB0 => {
                self.reset(Target::B, 6);
                8
            }
            0xB1 => {
                self.reset(Target::C, 6);
                8
            }
            0xB2 => {
                self.reset(Target::D, 6);
                8
            }
            0xB3 => {
                self.reset(Target::E, 6);
                8
            }
            0xB4 => {
                self.reset(Target::H, 6);
                8
            }
            0xB5 => {
                self.reset(Target::L, 6);
                8
            }
            0xB6 => {
                self.reset(Target::HL, 6);
                16
            }
            0xB7 => {
                self.reset(Target::A, 6);
                8
            }
            0xB8 => {
                self.reset(Target::B, 7);
                8
            }
            0xB9 => {
                self.reset(Target::C, 7);
                8
            }
            0xBA => {
                self.reset(Target::D, 7);
                8
            }
            0xBB => {
                self.reset(Target::E, 7);
                8
            }
            0xBC => {
                self.reset(Target::H, 7);
                8
            }
            0xBD => {
                self.reset(Target::L, 7);
                8
            }
            0xBE => {
                self.reset(Target::HL, 7);
                16
            }
            0xBF => {
                self.reset(Target::A, 7);
                8
            }
            0xC0 => {
                self.set(Target::B, 0);
                8
            }
            0xC1 => {
                self.set(Target::C, 0);
                8
            }
            0xC2 => {
                self.set(Target::D, 0);
                8
            }
            0xC3 => {
                self.set(Target::E, 0);
                8
            }
            0xC4 => {
                self.set(Target::H, 0);
                8
            }
            0xC5 => {
                self.set(Target::L, 0);
                8
            }
            0xC6 => {
                self.set(Target::HL, 0);
                16
            }
            0xC7 => {
                self.set(Target::A, 0);
                8
            }
            0xC8 => {
                self.set(Target::B, 1);
                8
            }
            0xC9 => {
                self.set(Target::C, 1);
                8
            }
            0xCA => {
                self.set(Target::D, 1);
                8
            }
            0xCB => {
                self.set(Target::E, 1);
                8
            }
            0xCC => {
                self.set(Target::H, 1);
                8
            }
            0xCD => {
                self.set(Target::L, 1);
                8
            }
            0xCE => {
                self.set(Target::HL, 1);
                16
            }
            0xCF => {
                self.set(Target::A, 1);
                8
            }
            0xD0 => {
                self.set(Target::B, 2);
                8
            }
            0xD1 => {
                self.set(Target::C, 2);
                8
            }
            0xD2 => {
                self.set(Target::D, 2);
                8
            }
            0xD3 => {
                self.set(Target::E, 2);
                8
            }
            0xD4 => {
                self.set(Target::H, 2);
                8
            }
            0xD5 => {
                self.set(Target::L, 2);
                8
            }
            0xD6 => {
                self.set(Target::HL, 2);
                16
            }
            0xD7 => {
                self.set(Target::A, 2);
                8
            }
            0xD8 => {
                self.set(Target::B, 3);
                8
            }
            0xD9 => {
                self.set(Target::C, 3);
                8
            }
            0xDA => {
                self.set(Target::D, 3);
                8
            }
            0xDB => {
                self.set(Target::E, 3);
                8
            }
            0xDC => {
                self.set(Target::H, 3);
                8
            }
            0xDD => {
                self.set(Target::L, 3);
                8
            }
            0xDE => {
                self.set(Target::HL, 3);
                16
            }
            0xDF => {
                self.set(Target::A, 3);
                8
            }
            0xE0 => {
                self.set(Target::B, 4);
                8
            }
            0xE1 => {
                self.set(Target::C, 4);
                8
            }
            0xE2 => {
                self.set(Target::D, 4);
                8
            }
            0xE3 => {
                self.set(Target::E, 4);
                8
            }
            0xE4 => {
                self.set(Target::H, 4);
                8
            }
            0xE5 => {
                self.set(Target::L, 4);
                8
            }
            0xE6 => {
                self.set(Target::HL, 4);
                16
            }
            0xE7 => {
                self.set(Target::A, 4);
                8
            }
            0xE8 => {
                self.set(Target::B, 5);
                8
            }
            0xE9 => {
                self.set(Target::C, 5);
                8
            }
            0xEA => {
                self.set(Target::D, 5);
                8
            }
            0xEB => {
                self.set(Target::E, 5);
                8
            }
            0xEC => {
                self.set(Target::H, 5);
                8
            }
            0xED => {
                self.set(Target::L, 5);
                8
            }
            0xEE => {
                self.set(Target::HL, 5);
                16
            }
            0xEF => {
                self.set(Target::A, 5);
                8
            }
            0xF0 => {
                self.set(Target::B, 6);
                8
            }
            0xF1 => {
                self.set(Target::C, 6);
                8
            }
            0xF2 => {
                self.set(Target::D, 6);
                8
            }
            0xF3 => {
                self.set(Target::E, 6);
                8
            }
            0xF4 => {
                self.set(Target::H, 6);
                8
            }
            0xF5 => {
                self.set(Target::L, 6);
                8
            }
            0xF6 => {
                self.set(Target::HL, 6);
                16
            }
            0xF7 => {
                self.set(Target::A, 6);
                8
            }
            0xF8 => {
                self.set(Target::B, 7);
                8
            }
            0xF9 => {
                self.set(Target::C, 7);
                8
            }
            0xFA => {
                self.set(Target::D, 7);
                8
            }
            0xFB => {
                self.set(Target::E, 7);
                8
            }
            0xFC => {
                self.set(Target::H, 7);
                8
            }
            0xFD => {
                self.set(Target::L, 7);
                8
            }
            0xFE => {
                self.set(Target::HL, 7);
                16
            }
            0xFF => {
                self.set(Target::A, 7);
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
                self.registers.set_half((self.registers.a & 0xF) + ((value as u8) & 0xF) > 0xF);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.b & 0xF) + ((value as u8) & 0xF) > 0xF);                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.c & 0xF) + ((value as u8) & 0xF) > 0xF);                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.d & 0xF) + ((value as u8) & 0xF) > 0xF);                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.e & 0xF) + ((value as u8) & 0xF) > 0xF);                
                self.registers.e = new_value;
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.h & 0xF) + ((value as u8) & 0xF) > 0xF);                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.l & 0xF) + ((value as u8) & 0xF) > 0xF);                
                self.registers.l = new_value;
            }
            Target::SP => {
                let (new_value, did_overflow) = self.registers.sp.overflowing_add(value as u16);

                self.registers.set_zero(false);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.sp & 0xF) + ((value as u16) & 0xF) > 0xF);                
                self.registers.set_carry((self.registers.sp & 0xFF) + ((value as u16) & 0xFF) > 0xFF);
                
                self.registers.sp = new_value;
            }
            Target::HL => {
                let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value as u16);

                
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_hl() & 0xFFF) + ((value as u16) & 0xFFF) > 0xFFF);
                self.registers.set_carry(did_overflow);

                self.registers.set_hl(new_value);
            }
            Target::DE => {
                let (new_value, did_overflow) = self.registers.get_de().overflowing_add(value as u16);


                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_de() & 0xFFF) + ((value as u16) & 0xFFF) > 0xFFF);
                
                self.registers.set_de(new_value);
            }
            Target::BC => {
                let (new_value, did_overflow) = self.registers.get_bc().overflowing_add(value as u16);


                self.registers.set_carry(did_overflow);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.get_bc() & 0xFFF) + ((value as u16) & 0xFFF) > 0xFFF);
                
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
                self.registers.set_half(((self.registers.a & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.b & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.c & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.d & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.e & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.e = new_value;
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.h & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry(did_overflow);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.l & 0xF) as i8) - ((value & 0xF) as i8) < 0);
                
                self.registers.l = new_value;
            }

            _ => {}
        }
    }
    fn adc(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let c = if self.registers.get_carry() { 1 } else { 0 };
                let new_value = self.registers.a.wrapping_add(value as u8).wrapping_add(c);

                self.registers.set_zero(new_value == 0);
                self.registers.set_carry((self.registers.a as u16) + (value as u16) + (c as u16) > 0xFF);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.a & 0xF) + (((value as u8) & 0xF) + (c as u8)) > 0xF);
                
                self.registers.a = new_value;
            }
            
            _ => {}
        }
    }
    fn sbc(&mut self, target: Target, value: usize){
        
        match target{
            Target::A => {
                let c = if self.registers.get_carry() { 1 } else { 0 };
                let new_value = self.registers.a.wrapping_sub(value as u8).wrapping_sub(c);
                self.registers.set_zero(new_value == 0);
                self.registers.set_carry((self.registers.a as i16) < (value as i16) + (c as i16));
                self.registers.set_sub(true);
                self.registers.set_half((self.registers.a & 0xF) < ((value as u8) & 0xF) + c);
                
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
        self.registers.set_carry(false);

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
                self.registers.set_half(((self.registers.a & 0xF) as i8) - ((value & 0xF) as i8) < 0);

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
                self.registers.set_half((self.registers.a & 0xF) + ((value as u8) & 0xF) > 0xF);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.b & 0xF) + ((value as u8) & 0xF) > 0xF);

                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.c & 0xF) + ((value as u8) & 0xF) > 0xF);

                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.d & 0xF) + ((value as u8) & 0xF) > 0xF);

                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.e & 0xF) + ((value as u8) & 0xF) > 0xF);

                self.registers.e = new_value;
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.h & 0xF) + ((value as u8) & 0xF) > 0xF);
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_add(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(false);
                self.registers.set_half((self.registers.l & 0xF) + ((value as u8) & 0xF) > 0xF);
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
                self.registers.set_half(((self.registers.a as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
                self.registers.a = new_value;
            }
            Target::B => {
                let (new_value, did_overflow) = self.registers.b.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.b as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
                self.registers.b = new_value;
            }
            Target::C => {
                let (new_value, did_overflow) = self.registers.c.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.c as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
                self.registers.c = new_value;
            }
            Target::D => {
                let (new_value, did_overflow) = self.registers.d.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.d as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
                self.registers.d = new_value;
            }
            Target::E => {
                let (new_value, did_overflow) = self.registers.e.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.e as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
                self.registers.e = new_value;
            }
            Target::H => {
                let (new_value, did_overflow) = self.registers.h.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.h as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
                self.registers.h = new_value;
            }
            Target::L => {
                let (new_value, did_overflow) = self.registers.l.overflowing_sub(value as u8);

                self.registers.set_zero(new_value == 0);
                self.registers.set_sub(true);
                self.registers.set_half(((self.registers.l as i8) & 0xF) - ((value & 0xF) as i8) < 0);
                
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
        self.registers.a = (!self.registers.a) & 0xFF;
        self.registers.set_half(true);
        self.registers.set_sub(true);
    }
    fn bit(&mut self, target: Target, value: u8){ //value is bit number to toggle (0 - 7)
        match target{
            Target::A => {
                let z = self.registers.a & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::B => {
                let z = self.registers.b & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::C => {
                let z = self.registers.c & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::D => {
                let z = self.registers.d & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::E => {
                let z = self.registers.e & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::H => {
                let z = self.registers.h & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },
            Target::L => {
                let z = self.registers.l & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
                self.registers.set_half(true);
                self.registers.set_sub(false);
            },

            Target::HL => {//mem address
                let z = self.memory.rb(self.registers.get_hl()) & (1 << (value as u32)) == 0;
                self.registers.set_zero(z);
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
                let new_val = self.registers.a.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.a & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.a = new_val;
            },
            Target::B => {
                let new_val = self.registers.b.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.b & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.b = new_val;
            },
            Target::C => {
                let new_val = self.registers.c.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.c & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.c = new_val;
            },
            Target::D => {
                let new_val = self.registers.d.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.d & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.d = new_val;
            },
            Target::E => {
                let new_val = self.registers.e.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.e & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.e = new_val;
            },
            Target::H => {
                let new_val = self.registers.h.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.h & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.h = new_val;
            },
            Target::L => {
                let new_val = self.registers.l.wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.l & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.l = new_val;
            },

            Target::HL => {//mem address
                let new_val = self.memory.rb(self.registers.get_hl()).wrapping_shr(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.memory.rb(self.registers.get_hl()) & 0x1 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.memory.wb(self.registers.get_hl(), new_val);
            }, 
            _ => {}

        }
    }
    fn sla(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                let new_val = self.registers.a.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.a & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.a = new_val;
            },
            Target::B => {
                let new_val = self.registers.b.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.b & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.b = new_val;
            },
            Target::C => {
                let new_val = self.registers.c.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.c & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.c = new_val;
            },
            Target::D => {
                let new_val = self.registers.d.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.d & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.d = new_val;
            },
            Target::E => {
                let new_val = self.registers.e.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.e & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.e = new_val;
            },
            Target::H => {
                let new_val = self.registers.h.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.h & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.h = new_val;
            },
            Target::L => {
                let new_val = self.registers.l.wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.l & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.l = new_val;
            },

            Target::HL => {//mem address
                let new_val = self.memory.rb(self.registers.get_hl()).wrapping_shl(1);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.memory.rb(self.registers.get_hl()) & 0x80 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.memory.wb(self.registers.get_hl(), new_val);
            }, 
            _ => {}

        }
    }
    fn sra(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                let new_val = self.registers.a.wrapping_shr(1).wrapping_add(self.registers.a & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.a & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.a = new_val;
            },
            Target::B => {
                let new_val = self.registers.b.wrapping_shr(1).wrapping_add(self.registers.b & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.b & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.b = new_val;
            },
            Target::C => {
                let new_val = self.registers.c.wrapping_shr(1).wrapping_add(self.registers.c & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.c & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.c = new_val;
            },
            Target::D => {
                let new_val = self.registers.d.wrapping_shr(1).wrapping_add(self.registers.d & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.d & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.d = new_val;
            },
            Target::E => {
                let new_val = self.registers.e.wrapping_shr(1).wrapping_add(self.registers.e & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.e & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.e = new_val;
            },
            Target::H => {
                let new_val = self.registers.h.wrapping_shr(1).wrapping_add(self.registers.h & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.h & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.h = new_val;
            },
            Target::L => {
                let new_val = self.registers.l.wrapping_shr(1).wrapping_add(self.registers.l & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(self.registers.l & 0x01 != 0);
                self.registers.set_sub(false);
                self.registers.set_half(false);
                self.registers.l = new_val;
            },

            Target::HL => {//mem address
                let hl = self.memory.rb(self.registers.get_hl());
                let new_val = hl.wrapping_shr(1).wrapping_add(hl & 0x80);
                self.registers.set_zero(new_val == 0);
                self.registers.set_carry(hl & 0x01 != 0);
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
                let val = (self.registers.a >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.a & 0x1 != 0);
                self.registers.a = val;
            },
            Target::B => {
                let val = (self.registers.b >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.b & 0x1 != 0);
                self.registers.b = val;
            },
            Target::C => {
                let val = (self.registers.c >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.c & 0x1 != 0);
                self.registers.c = val;
            },
            Target::D => {
                let val = (self.registers.d >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.d & 0x1 != 0);
                self.registers.d = val;
            },
            Target::E => {
                let val = (self.registers.e >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.e & 0x1 != 0);
                self.registers.e = val;
            },
            Target::H => {
                let val = (self.registers.h >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.h & 0x1 != 0);
                self.registers.h = val;
            },
            Target::L => {
                let val = (self.registers.l >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.l & 0x1 != 0);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let hl = self.memory.rb(self.registers.get_hl());
                let val = (hl >> 1) + if self.registers.get_carry() { 0x80 } else { 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(hl & 0x1 != 0);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn rl(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {

                let val = (self.registers.a << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.a & 0x80 != 0);
                self.registers.a = val;
            },
            Target::B => {
                let val = (self.registers.b << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.b & 0x80 != 0);
                self.registers.b = val;
            },
            Target::C => {
                let val = (self.registers.c << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.c & 0x80 != 0);
                self.registers.c = val;
            },
            Target::D => {
                let val = (self.registers.d << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.d & 0x80 != 0);
                self.registers.d = val;
            },
            Target::E => {
                let val = (self.registers.e << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.e & 0x80 != 0);
                self.registers.e = val;
            },
            Target::H => {
                let val = (self.registers.h << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.h & 0x80 != 0);
                self.registers.h = val;
            },
            Target::L => {
                let val = (self.registers.l << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.l & 0x80 != 0);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let hl = self.memory.rb(self.registers.get_hl());
                let val = (hl << 1) + if self.registers.get_carry() { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(hl & 0x80 != 0);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn rla(&mut self){

        let val = self.registers.a.wrapping_shl(1) + if self.registers.get_carry() { 0x1 }else{ 0 };

        self.registers.set_zero(false);
        self.registers.set_half(false);
        self.registers.set_sub(false);
        self.registers.set_carry(self.registers.a & 0x80 != 0);
        self.registers.a = val;
    }
    fn rra(&mut self){

        let val = self.registers.a.wrapping_shr(1) + if self.registers.get_carry() { 0x80 }else{ 0 };
        self.registers.set_zero(false);
        self.registers.set_half(false);
        self.registers.set_sub(false);
        self.registers.set_carry(self.registers.a & 0x1 != 0);
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
    fn rlc(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                let c = self.registers.a & 0x80 == 0x80;
                let val = (self.registers.a << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.a & 0x80 != 0);
                self.registers.a = val;
            },
            Target::B => {
                let c = self.registers.b & 0x80 == 0x80;
                let val = (self.registers.b << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.b & 0x80 != 0);
                self.registers.b = val;
            },
            Target::C => {
                let c = self.registers.c & 0x80 == 0x80;
                let val = (self.registers.c << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.c & 0x80 != 0);
                self.registers.c = val;
            },
            Target::D => {
                let c = self.registers.d & 0x80 == 0x80;
                let val = (self.registers.d << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.d & 0x80 != 0);
                self.registers.d = val;
            },
            Target::E => {
                let c = self.registers.e & 0x80 == 0x80;
                let val = (self.registers.e << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.e & 0x80 != 0);
                self.registers.e = val;
            },
            Target::H => {
                let c = self.registers.h & 0x80 == 0x80;
                let val = (self.registers.h << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.h & 0x80 != 0);
                self.registers.h = val;
            },
            Target::L => {
                let c = self.registers.l & 0x80 == 0x80;
                let val = (self.registers.l << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.l & 0x80 != 0);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let hl = self.memory.rb(self.registers.get_hl());
                let c = hl & 0x80 == 0x80;
                let val = (hl << 1) + if c { 0x1 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(hl & 0x80 != 0);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
            _ => {}

        }
    }
    fn rrc(&mut self, target: Target){ //value is bit number to set (0 - 7)
        match target{
            Target::A => {
                let c = self.registers.a & 0x1 == 0x1;
                let val = (self.registers.a >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.a & 0x1 != 0);
                self.registers.a = val;
            },
            Target::B => {
                let c = self.registers.b & 0x1 == 0x1;
                let val = (self.registers.b >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.b & 0x1 != 0);
                self.registers.b = val;
            },
            Target::C => {
                let c = self.registers.c & 0x1 == 0x1;
                let val = (self.registers.c >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.c & 0x1 != 0);
                self.registers.c = val;
            },
            Target::D => {
                let c = self.registers.d & 0x1 == 0x1;
                let val = (self.registers.d >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.d & 0x1 != 0);
                self.registers.d = val;
            },
            Target::E => {
                let c = self.registers.e & 0x1 == 0x1;
                let val = (self.registers.e >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.e & 0x1 != 0);
                self.registers.e = val;
            },
            Target::H => {
                let c = self.registers.h & 0x1 == 0x1;
                let val = (self.registers.h >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.h & 0x1 != 0);
                self.registers.h = val;
            },
            Target::L => {
                let c = self.registers.l & 0x1 == 0x1;
                let val = (self.registers.l >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(self.registers.l & 0x1 != 0);
                self.registers.l = val;
            },

            Target::HL => {//mem address
                let hl = self.memory.rb(self.registers.get_hl());
                let c = hl & 0x1 == 0x1;
                let val = (hl >> 1) + if c { 0x80 }else{ 0 };
                self.registers.set_zero(val == 0);
                self.registers.set_half(false);
                self.registers.set_sub(false);
                self.registers.set_carry(hl & 0x1 != 0);
                self.memory.wb(self.registers.get_hl(), val);
            }, 
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

