mod cpu;
mod _cpu;


use cpu::*;

use std::io::{stdin, stdout, Write, Read};
use std::thread::sleep_ms;
extern crate sdl2; 



use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;



const WIDTH : u32 = 640;
const HEIGHT : u32 = 320;

fn main(){
    /*let scale_x = (WIDTH / 160) as u32;
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

    let mut event_pump = sdl_context.event_pump().unwrap(); */

    //println!("Loaded screen!");
    let mut cpu = CPU::new();
    cpu.load_rom();
    println!("Loaded ROM!");

    //cpu.memory.set_initial();
    println!("Set initial!");

    let mut step : isize = 0;
    println!("Debugging! \n\n");
    'running: loop{
        
       /* //clr screen
        canvas.set_draw_color(Color::RGB(0,0,0));
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
*/
        cpu.step();
        step += 1;



        if cpu.memory.rb(0xFF02) == 0x81{
            let c : char = cpu.memory.rb(0xFF01) as char;
            println!("Output: {}",c);
            cpu.memory.wb(0xFF02, 0x0);
            panic!("):");
        }
        if cpu.registers.pc >= 0x100{
            break 'running;
        }
        /*for x in 0..160{
            for y in 0..144{
                
                
                let pixel = cpu.memory.gpu.screen_buffer[x * y * 4];
                canvas.set_draw_color(Color::RGB(pixel, pixel, pixel));
                 
                canvas.fill_rect(Rect::new((x * scale_x as usize) as i32, (y * scale_y as usize) as i32, scale_x, scale_y)).unwrap();
                
            }
            
        }*/
        //canvas.present();
        //let _ = stdin().read(&mut [0u8]).unwrap();
        //sleep_ms(750);
    }
    println!("Finished!");
    
}
