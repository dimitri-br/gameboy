mod cpu;
mod _cpu;


use cpu::*;

use std::io::{stdin, stdout, Write, Read};
use std::thread::sleep_ms;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_rom();
    let mut step = 0;
    println!("Debugging! \n\n");
    loop{
        if cpu.registers.pc >= 0x100 * 2{
            break;
        }
        cpu.step();
        step += 1;
        if cpu.memory.rb(0xFF02) == 0x81{
            let c : char = cpu.memory.rb(0xFF01) as char;
            println!("{}",c);
            cpu.memory.wb(0xFF02, 0x0);
            panic!("):");
        }
        /*print!("\n\nPress any key to continue...");
        stdout().flush();
        stdin().read(&mut [0x0]).unwrap();*/
    }
    
}
