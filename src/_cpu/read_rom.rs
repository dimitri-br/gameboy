//file reads rom from file
use std::fs::File;
use std::io::{Read, stdin};

pub struct ROM{
    pub path: String,
    pub content: [u8; 0x7FFF] //ROM buffer - TODO - INCREASE!!! :D
}

impl ROM{
    pub fn new(path: String) -> Self{
        ROM {
            path: path,
            content: [0x0; 0x7FFF]
        }
    }
    pub fn load(&mut self){
        let mut file = File::open(&self.path).expect("Error opening file");
        let mut buffer = [0u8; 0x7FFF];
        let _bytes_read = if let Ok(bytes_read) = file.read(&mut buffer) {
            bytes_read
        } else {
            0
        };
        self.content = buffer;
        
        println!("• Loaded ROM: {}", self.path);
    }
}