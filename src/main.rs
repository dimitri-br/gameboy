mod cpu;
mod _cpu;


use cpu::*;

use std::io::{stdin, Read};
use std::thread;
extern crate sdl2; 



use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;



const WIDTH : u32 = 320;
const HEIGHT : u32 = 144*2;

fn main(){
    let mut trace_buffer = Vec::<String>::new();
    let scale_x = (WIDTH / 160) as u32;
    let scale_y = (HEIGHT / 144) as u32;
    //sdl and gfx
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let title = format!("GameBoy");
    let window = video_subsystem.window(&title, WIDTH, HEIGHT)
        .position_centered()
        .build()
        
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap(); 
    

    //println!("Loaded screen!");
    let mut cpu = CPU::new();
    cpu.load_rom();
    println!("Loaded ROM!");

    cpu.memory.set_initial();
    
    

    //cpu.registers.set_af(0x01B0);
    //cpu.registers.set_bc(0x0013);
    //cpu.registers.set_de(0x00D8);
    //cpu.registers.set_hl(0x014D);
    //cpu.registers.sp = 0xFFFE;

    cpu.registers.pc = 0x0;
    

    println!("Set initial!");


    println!("Debugging! \n\n");
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
        //thread::sleep(std::time::Duration::from_nanos(((4000000 / 4) / 60) * cpu.delay as u64));
        
    }
    println!("Finished!");
    
    save(trace_buffer);

    
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