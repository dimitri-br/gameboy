use std::collections::HashMap;
use super::memory;

pub struct GPU{
    pub vram: [u8; 0x2000],
    reg: [u8; 0x2000],
    oam: [u8; 0xFF],

    pal: [[u8; 4]; 3],
    tileset: [[[u8; 8]; 8]; 384],
    mode: GPU_Mode,
    mode_clock: u16,
    curline: u8,
    curscan: u16,
    scy: u8,
    scx: u8,
    winx: u8,
    winy: u8,
    bg_tile_base: u16,
    bg_map_base: u16,
    win_map_base: u16,
    lcd_on: bool,
    win_on: bool,
    bg_on: bool,
    obj_on: bool,
    scanline: u8,
    obj_data: Vec::<HashMap::<String, i8>>,
    obj_data_sorted: Vec::<HashMap::<String, i8>>,
    scanrow: [u8; 160],
    obj_size: u8,
    int_fired: u8,
    raster: u8,
    ints: u8,

    pub screen_buffer: [u8; 144 * 160 * 4],

}

impl GPU {
    pub fn new() -> Self{
        GPU {
            vram: [0x0; 0x2000],
            reg: [0x0; 0x2000],
            oam: [0x0; 0xFF],
            pal: [[0x0; 4]; 3],
            tileset: [[[0x0; 8]; 8]; 384],
            obj_data: Vec::<HashMap::<String, i8>>::new(),
            obj_data_sorted: Vec::<HashMap::<String, i8>>::new(),
            scanrow: [0x0; 160],
            screen_buffer: [0x0; 144 * 160 * 4],


            mode: GPU_Mode::VBlank,
            mode_clock: 0,
            curline: 0,
            curscan: 0,
            scx: 0,
            scy: 0,
            winx: 0,
            winy: 0,
            bg_tile_base: 0x0,
            bg_map_base: 0x1800,
            win_map_base: 0x1800,
            lcd_on: false,
            win_on: false,
            bg_on: false,
            obj_on: false,
            
            obj_size: 0,
            scanline: 0,
            
            raster: 0,
            int_fired: 0,
            ints: 0,
        }
    }
}

impl GPU{

    pub fn rb(&mut self, mut address: usize) -> u8{
        address = address - 0xFF40;
        match address{
            0 => {
                return if self.lcd_on { 0x80 }else{ 0 } | if self.win_map_base==0x1C00 { 0x40 }else{ 0 } | if self.win_on { 0x20 }else{ 0 } | if self.bg_tile_base == 0x0000 { 0x10 }else{ 0 } | if self.bg_map_base == 0x1C00 { 0x8 } else{ 0 } | if self.obj_size != 0 { 0x4 }else{ 0 } | if self.obj_on { 0x2 }else{ 0 } | if self.bg_on { 0x1 }else { 0 }
            }
            1 => {
                let intf = self.int_fired as u8;
                self.int_fired = 0;
                return intf << 3 | if self.curline==self.raster { 4 }else{ 0 } | self.mode as u8
            }
            2 => {
                self.scy
            }
            3 => {
                self.scx
            }
            4 => {
                self.curline
            }
            5 => {
                self.raster
            }
            10 => {
                self.winy
            }
            11 => {
                self.winx + 7
            }
            _ => { self.reg[address] }
        }
    }
    pub fn wb(&mut self, mut address: usize, value: u8, locations: [u8; 160]){
        let address = address - 0xFF40;
        self.reg[address] = value;
        match address{
            1 => {
                self.lcd_on = if (value & 0x80) != 0 { true }else{ false };
                self.win_map_base = if (value & 0x40) != 0 { 0x1C00 }else{ 0x1800 };
                self.win_on = if (value & 0x20) != 0 { true }else{ false };
                self.bg_tile_base = if (value & 0x10) != 0 { 0x0 }else{ 0x0800 };
                self.bg_map_base = if (value & 0x8) != 0 { 0x1C00 }else{ 0x1800 };
                self.obj_size = if (value & 0x4) != 0 { 1 }else{ 0 };
                self.obj_on = if (value & 0x2) != 0 { true }else{ false };
                self.bg_on = if (value & 0x1) != 0 { true } else { false };
            }
            1 => {
                self.ints = (value >> 3) & 15;
            }
            2 => {
                self.scy = value;
            }
            3 => {
                self.scx = value;
            }
            5 => {
                self.raster = value;
            }
            6 => {
                let mut v = 0;
                for i in 0..160{
                    v = locations[i];
                    self.oam[i as usize] = v;
                    self.update_oam(0xFE00 + i as u16, v);
                }
            }
            7 => {
                for i in 0..4{
                    match (value >> (i * 2)) & 3{
                        0 => { self.pal[0][i] = 255; }
                        1 => { self.pal[0][i] = 192;}
                        2 => { self.pal[0][i] = 96;}
                        3 => { self.pal[0][i] = 0}
                        _ => { 0; }
                    }
                }
            }
            8 => {
                for i in 0..4{
                    match (value >> (i * 2)) & 3{
                        0 => { self.pal[1][i] = 255; }
                        1 => { self.pal[1][i] = 192;}
                        2 => { self.pal[1][i] = 96;}
                        3 => { self.pal[1][i] = 0}
                        _ => { 0; }
                    }
                }
            }
            9 => {
                for i in 0..4{
                    match (value >> (i * 2)) & 3{
                        0 => { self.pal[2][i] = 255; }
                        1 => { self.pal[2][i] = 192;}
                        2 => { self.pal[2][i] = 96;}
                        3 => { self.pal[2][i] = 0}
                        _ => { 0; }
                    }
                }
            }
            10 => {
                self.winy = value;
            }
            11 => {
                self.winx = value + 7;
            }
            _ => { 0; }
        }
    }
}
impl GPU{
    pub fn reset(&mut self){
        self.tileset = [[[0x0; 8]; 8]; 384];
        self.bg_map_base = 0;
        self.bg_tile_base = 0;
        self.win_map_base = 0;
        self.mode_clock = 0;
        self.curline = 0;
        self.curscan = 0;
        self.scx = 0;
        self.scy = 0;
        self.winx = 0;
        self.winy = 0;
        self.ints = 0;
        self.int_fired = 0;
        self.raster = 0;

        self.bg_on = false;
        self.win_on = false;
        self.lcd_on = false;
        self.obj_on = false;


        self.scanrow = [0x0; 160];
        for i in 0..40{
            self.obj_data[i].entry("y".to_string()).or_insert(-16);
            self.obj_data[i].entry("x".to_string()).or_insert(-8);
            self.obj_data[i].entry("tile".to_string()).or_insert(0);
            self.obj_data[i].entry("palette".to_string()).or_insert(0);
            self.obj_data[i].entry("yflip".to_string()).or_insert(0);
            self.obj_data[i].entry("xflip".to_string()).or_insert(0);
            self.obj_data[i].entry("prio".to_string()).or_insert(0);
            self.obj_data[i].entry("num".to_string()).or_insert(i as i8);
        }

        self.bg_tile_base = 0x0000;
        self.win_map_base = 0x1800;
        self.bg_map_base = 0x1800;
    }
    pub fn update_tileset(&mut self, address: u16, value: u8){
        let mut address = address;
        if address & 1 != 0 { 
            address -= 1;
        }
        let tile = (address >> 4) & 511;
        let y = (address >> 1) & 7;

        let mut sx = 0;
        for x in 0..8{
            sx = 1 << (7 - x);
            self.tileset[tile as usize][y as usize][x as usize] = (if self.vram[address as usize] & sx != 0 {1}else{0}) + (if self.vram[address as usize + 1] & sx != 0 {2}else{0});
        }
    }
    pub fn update_oam(&mut self, mut address: u16, value: u8){
        address -= 0xFE00;
        let obj = address >> 2;
        if obj < 40{
            match address & 0x3{
                0 => {self.obj_data[obj as usize].entry("y".to_string()).or_insert(value as i8 - 16);}
                1 => {self.obj_data[obj as usize].entry("x".to_string()).or_insert(value as i8 - 8);}
                2 => {
                    if self.obj_size != 0{
                        self.obj_data[obj as usize].entry("tile".to_string()).or_insert((address & 0xFE) as i8);
                    }else{
                        self.obj_data[obj as usize].entry("tile".to_string()).or_insert(value as i8);
                    }
                }
                3 => {
                    self.obj_data[obj as usize].entry("palette".to_string()).or_insert(if (value & 0x10) != 0 { 1 }else{ 0 });
                    self.obj_data[obj as usize].entry("xflip".to_string()).or_insert(if (value & 0x20) != 0 { 1 }else{ 0 });
                    self.obj_data[obj as usize].entry("yflip".to_string()).or_insert(if (value & 0x40) != 0 { 1 }else{ 0 });
                    self.obj_data[obj as usize].entry("prio".to_string()).or_insert(if (value & 0x80) != 0 { 1 }else{ 0 });
                }
                _ => {}
            }
        }
    }
}
impl GPU {
    pub fn step(&mut self, t: u8) -> u8{
        self.mode_clock += t as u16;
        match self.mode{
            GPU_Mode::OAM_Read => {
                if self.mode_clock >= 20{
                    self.mode_clock = 0;
                    self.mode = GPU_Mode::VRAM_Read;
                }
            }
            GPU_Mode::VRAM_Read => {
                if self.mode_clock >= 43{
                    self.mode_clock = 0;
                    self.mode = GPU_Mode::HBlank;
                    
                    //Render scanline to video buffer here
                    if (self.ints & 0x1) != 0 { self.int_fired |= 2; return 2}
                    self.render_scan();
                    
                    
                }
            }
            GPU_Mode::HBlank => {
                if self.mode_clock >= 51{
                    
                    if self.mode_clock == 143{
                        self.mode = GPU_Mode::VBlank;
                        //canvas put on screen
                        if (self.ints & 0x2) != 0 { self.int_fired |= 2; return 2}
                    }else{
                        self.mode = GPU_Mode::OAM_Read;
                        if (self.ints & 0x4) != 0 { self.int_fired |= 4; return 2}
                    }
                    self.curline += 1;
                    if self.curline == self.raster{
                        if (self.ints & 0x8) != 0 { self.int_fired |= 8; return 2}
                    }
                    self.mode_clock = 0;
                    
                    self.curscan += 640;
                }
            }
            GPU_Mode::VBlank => {
                if self.mode_clock >= 114{
                    self.mode_clock = 0;
                    self.curline += 1;
                    if self.curline > 153{
                        self.mode = GPU_Mode::OAM_Read;
                        self.curline = 0;
                        self.curscan = 0;
                    }
                    if (self.ints & 0x4) != 0 { self.int_fired |= 4; return 2}
                }
            }
        }
        0
    }
    pub fn render_scan(&mut self){
        if self.lcd_on{
            if self.bg_on{
                let mut linebase = self.curscan;
                let mapbase = self.bg_map_base + ((((self.curline+self.scy)&255)<<3) >> 5) as u16;
                let y = (self.curline + self.scy) & 7;
                let mut x = self.scx & 7;
                let mut t = (self.scx >> 3) & 31;
                let pixel = 0;
                let mut w = 160;
                if self.bg_tile_base != 0{
                    let mut tile = self.vram[(mapbase + t as u16) as usize];
                    if tile< 128 { tile = tile.wrapping_add(128); tile = tile.wrapping_add(128); };
                    let mut tile_row = self.tileset[tile as usize][y as usize];
                    while w != 0{
                        self.scanrow[160 - x as usize] = tile_row[x as usize];
                        self.screen_buffer[linebase as usize + 3] = self.pal[0][tile_row[x as usize] as usize];
                        x += 1;
                        if x == 8{
                            t = (t + 1) & 31;
                            x = 0;
                            tile = self.vram[(mapbase + t as u16) as usize];
                            if tile< 128 { tile = tile.wrapping_add(128); tile = tile.wrapping_add(128); };
                            tile_row = self.tileset[tile as usize][y as usize];
                        }
                        linebase += 4;
                        w -= 1;

                    }
                }else{
                    let mut tile_row = self.tileset[self.vram[(mapbase + t as u16) as usize] as usize][y as usize];
                    while w != 0{
                        self.scanrow[160 - x as usize] = tile_row[x as usize];
                        self.screen_buffer[linebase as usize + 3] = self.pal[0][tile_row[x as usize] as usize];
                        x += 1;
                        if x == 8{
                            t = (t + 1) & 31;
                            x = 0;
                            tile_row = self.tileset[self.vram[(mapbase + t as u16) as usize] as usize][y as usize];
                        }
                        linebase += 4;
                        w -= 1;
                    }
                }
                if self.obj_on{
                    let mut count = 0;
                    if self.obj_size != 0{
                        for i in 0..40{

                        }
                    }else{
                        let tile_row = 0;
                        let mut obj = HashMap::<String, i8>::new();
                        let pal = 0;
                        let pixel = 0;
                        let x = 0;
                        let linebase = self.curscan;
                        for i in 0..40{
                            //obj = self.obj_data[i];
                        }
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum GPU_Mode{
    OAM_Read,
    VRAM_Read,
    HBlank,
    VBlank
}