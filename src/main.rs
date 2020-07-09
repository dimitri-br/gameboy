mod cpu;
mod _cpu;

use std::env;

use cpu::*;

extern crate sdl2; 



use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;



const WIDTH : u32 = 320;
const HEIGHT : u32 = 144*2;

fn main(){
    
    //get rom to run (Can be run through drag n drop or cmd)
    let args: Vec<String> = env::args().collect();

    let rom = args[1].to_owned();


    let rom_name : Vec::<&str> = rom.split("/").collect();
    let rom_name = rom_name.last().unwrap();

    println!("• Starting...");
    let mut trace_buffer = Vec::<String>::new();
    let scale_x = (WIDTH / 160) as u32;
    let scale_y = (HEIGHT / 144) as u32;
    //sdl and gfx
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let title = format!("GameBoy - {}", rom_name);
    let window = video_subsystem.window(&title, WIDTH, HEIGHT)
        .opengl()

        .position_centered()
        .build()
        
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap(); 
    

    println!("• Setup Display!");
    let mut cpu = CPU::new();
    cpu.load_rom(rom.to_string());
    println!("• Loaded ROM Successfully!");

    for _ in 0..0x20000{
        cpu.memory.eram.push(0x0);
    }
    cpu.memory.rom_name = rom.to_string();
    cpu.memory.load_sram();
    //cpu.init(); //exit boot rom and set values
    cpu.registers.pc = 0x0; //start at 0














    'running: loop{
        
        //clr screen
        canvas.set_draw_color(Color::RGB(0,255/2,255/2));
        canvas.clear();
        //events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode  {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::Up) => {cpu.memory.keys.rows[1] &= 0xB}
                        Some(Keycode::Down) => {cpu.memory.keys.rows[1] &= 0x7}
                        Some(Keycode::Left) => {cpu.memory.keys.rows[1] &= 0xD}
                        Some(Keycode::Right) => {cpu.memory.keys.rows[1] &= 0xE}

                        Some(Keycode::Space) => {cpu.memory.keys.rows[0] &= 0xB}
                        Some(Keycode::Return) => {cpu.memory.keys.rows[0] &= 0x7;}
                        Some(Keycode::X) => {cpu.memory.keys.rows[0] &= 0xD}
                        Some(Keycode::Z) => {cpu.memory.keys.rows[0] &= 0xE}
                        _ => {}

                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode{
                        Some(Keycode::Up) => {cpu.memory.keys.rows[1] |= 0x4}
                        Some(Keycode::Down) => {cpu.memory.keys.rows[1] |= 0x8}
                        Some(Keycode::Left) => {cpu.memory.keys.rows[1] |= 0x2}
                        Some(Keycode::Right) => {cpu.memory.keys.rows[1] |= 0x1}

                        Some(Keycode::Space) => {cpu.memory.keys.rows[0] |= 0x4}
                        Some(Keycode::Return) => {cpu.memory.keys.rows[0] |= 0x8}
                        Some(Keycode::X) => {cpu.memory.keys.rows[0] |= 0x2}
                        Some(Keycode::Z) => {cpu.memory.keys.rows[0] |= 0x1}
                        _ => {}
                    }
                }
                _ => {}
            }
        } 
        if !cpu.pause{
            cpu.step();

        }else{
            println!("Halt!");
        }
        for trace in cpu.trace.iter(){
            trace_buffer.push(trace.clone());
        }
        

        



        
        if cpu.registers.pc > 0xFFFF{
            break 'running;
        }
        for x in 0..160{
            for y in 0..144{
                
                
                let p1 = cpu.memory.gpu.data[y * 160 * 3 + x * 3 + 0];
                let p2 = cpu.memory.gpu.data[y * 160 * 3 + x * 3 + 1];
                let p3 = cpu.memory.gpu.data[y * 160 * 3 + x * 3 + 2];
                canvas.set_draw_color(Color::RGB(p1, p2, p3));
                
                canvas.fill_rect(Rect::new((x * scale_x as usize) as i32, (y * scale_y as usize) as i32, scale_x, scale_y)).unwrap();
                
            }
            
        } 
        canvas.present();
        
    }
    println!("• Finished!");
    
    //save(trace_buffer);

    
}

fn save(trace_buffer: Vec::<String>){
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("trace.txt").unwrap();
    for line in trace_buffer.iter(){
        file.write(line.as_bytes()).unwrap();
    }
    file.flush().unwrap();
    println!("Saved file!")
}
/*
use std::fs::File;
use std::io::Write;
fn save(buf: Vec::<String>, file_name: &str){
    let mut file = File::create(file_name).expect("Error creating file");

    let mut counter = 1;
    for line in buf.iter(){
        file.write(line.as_bytes()).unwrap();
        if counter % 16 == 0{
            file.write(b"\n").unwrap();
        }else{
            file.write(b" ").unwrap();
        }
        counter += 1;
        
    }
    file.flush().unwrap();
    println!("Saved file: {}", file_name);

}
*/