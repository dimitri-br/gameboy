//file reads rom from file
use std::fs::File;
use std::io::{Read, stdin};

pub struct ROM{
    pub path: String,
    pub content: Vec::<u8> //ROM buffer - TODO - INCREASE!!! :D
}

impl ROM{
    pub fn new(path: String) -> Self{
        ROM {
            path: path,
            content: Vec::<u8>::new(),
        }
    }
    pub fn load(&mut self){
        let mut file = File::open(&self.path).expect("Error opening file");
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();
        self.content = buffer;
        
        println!("• Loaded ROM: {}", self.path);
    }
}