pub struct GPU{
    vram: [u8; 0x2000],
    reg: [u8; 0x2000],
    oam: [u8; 0xFF],
    pub screen: [u8; 144 * 160],
    pal: [[u8; 4]; 3],
    tileset: [[[u8; 8]; 8]; 384],
    mode: GPU_Mode,
    mode_clock: u16,
    line: u8,
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
    obj_size: u8,
    int_fired: bool,
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
            screen: [0x0; 144 * 160],
            pal: [[0x0; 4]; 3],
            tileset: [[[0x0; 8]; 8]; 384],
            mode: GPU_Mode::VBlank,
            mode_clock: 0,
            line: 0,
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
            screen_buffer: [0x0; 144 * 160 * 4],
            raster: 0,
            int_fired: false,
            ints: 0,
        }
    }
}

impl GPU{
    fn set_int_fired(&mut self, value: bool){
        self.int_fired = value;
    }
    pub fn rb(&self, mut address: usize) -> u8{
        address = address - 0xFF40;
        match address{
            0 => {
                return if self.lcd_on { 0x80 }else{ 0 } | if self.win_map_base==0x1C00 { 0x40 }else{ 0 } | if self.win_on { 0x20 }else{ 0 } | if self.bg_tile_base == 0x0000 { 0x10 }else{ 0 } | if self.bg_map_base == 0x1C00 { 0x8 } else{ 0 } | if self.obj_size != 0 { 0x4 }else{ 0 } | if self.obj_on { 0x2 }else{ 0 } | if self.bg_on { 0x1 }else { 0 }
            }
            1 => {
                let intf = self.int_fired as u8;
                return intf << 3 | if self.line==self.raster { 4 }else{ 0 } | self.mode as u8
            }
            2 => {
                self.scy
            }
            3 => {
                self.scx
            }
            4 => {
                self.line
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
    pub fn wb(&mut self, mut address: usize, value: u8){
        address = address - 0xFF40;
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
            2 => {
                self.scy = value;
            }
            3 => {
                self.scx = value;
            }
            5 => {
                self.raster = value;
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
                self.winx = value - 7;
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
    }
    pub fn update_tileset(&mut self, address: u16, value: u8){
        let address = address & 0x1FFE;
        let tile = (address >> 4) & 511;
        let y = (address >> 1) & 7;

        let mut sx = 0;
        for x in 0..8{
            sx = 1 << (7 - x);
            self.tileset[tile as usize][y as usize][x as usize] = (if self.vram[address as usize] & sx != 0 {1}else{0}) + (if self.vram[address as usize + 1] & sx != 0 {2}else{0});
        }
    }
}
impl GPU {
    pub fn step(&mut self, t: u8){
        self.mode_clock += t as u16;
        match self.mode{
            GPU_Mode::OAM_Read => {
                if self.mode_clock >= 80{
                    self.mode_clock = 0;
                    self.mode = GPU_Mode::VRAM_Read;
                }
            }
            GPU_Mode::VRAM_Read => {
                if self.mode_clock >= 172{
                    self.mode_clock = 0;
                    self.mode = GPU_Mode::HBlank;

                    //Render scanline to video buffer here
                    self.render_scan();
                }
            }
            GPU_Mode::HBlank => {
                if self.mode_clock >= 204{
                    self.mode_clock = 0;
                    self.line += 1;
                    if self.mode_clock == 143{
                        self.mode = GPU_Mode::VBlank;
                        //canvas put on screen
                    }else{
                        self.mode = GPU_Mode::OAM_Read;
                    }
                }
            }
            GPU_Mode::VBlank => {
                if self.mode_clock >= 456{
                    self.mode_clock = 0;
                    self.line += 1;
                    if self.line > 153{
                        self.mode = GPU_Mode::OAM_Read;
                        self.line = 0;
                    }
                }
            }
        }
    }
    pub fn render_scan(&mut self){
        let mut mapoffs : u16 = if self.bg_map_base == 1 {0x1C00}else{0x1800};
        mapoffs += (((self.line + self.scy) as u16) & 255) >> 3;
        let mut lineoff = self.scx >> 3;

        let y = (self.line + self.scy) & 7;

        let mut x = self.scx & 7;

        let mut canvasoff = self.line * 160 * 4;

        let mut color : [u8; 4] = [0x0; 4];
        let mut tile = self.vram[(mapoffs + lineoff as u16) as usize] as u16;
        if self.bg_tile_base == 1 && tile < 128 {tile += 256};
        for i in 0..160{
            color = self.pal[self.tileset[tile as usize][x as usize][y as usize] as usize];
            self.screen_buffer[canvasoff as usize + 1] = color[0];
            self.screen_buffer[canvasoff as usize + 2] = color[1];
            self.screen_buffer[canvasoff as usize + 3] = color[2];
            self.screen_buffer[canvasoff as usize + 4] = color[3];
            canvasoff += 4;

            x += 1;
            if x == 8{
                x = 0;
                lineoff = (lineoff + 1) & 31;
                tile = self.vram[(mapoffs + lineoff as u16) as usize] as u16;
                if self.bg_tile_base == 1 && tile < 128 {tile += 256};
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