pub struct GPU{
    vram: [u8; 0x2000],
    oam: [u8; 0xFF],
    pub screen: [u8; 144 * 160],
    pub pal: [u8; 4],
    pub palb: [u8; 4],
    pub bg_map: u16,
    tileset: [[[u8; 8]; 8]; 384],
    mode: GPU_Mode,
    mode_clock: u16,
    line: u8,
    scy: u8,
    scx: u8,

}

impl GPU {
    pub fn new() -> Self{
        GPU {
            vram: [0x0; 0x2000],
            oam: [0x0; 0xFF],
            screen: [0x0; 144 * 160],
            pal: [0x0; 4],
            palb: [0x0; 4],
            bg_map: 0,
            tileset: [[[0x0; 8]; 8]; 384],
            mode: GPU_Mode::VBlank,
            mode_clock: 0,
            line: 0,
            scx: 0,
            scy: 0,
        }
    }
}

impl GPU{
    pub fn rb_vram(&self, address: usize) -> u8{
        self.vram[address]
    }
    pub fn wb_vram(&mut self, address: usize, value: u8){
        self.vram[address] = value;
    }

    pub fn rb_oam(&self, address: usize) -> u8{
        self.oam[address]
    }
    pub fn wb_oam(&mut self, address: usize, value: u8){
        self.oam[address] = value;
    }
}
impl GPU{
    pub fn reset(&mut self){
        self.tileset = [[[0x0; 8]; 8]; 384];
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
        let mut mapoffs : u16 = if self.bg_map == 1 {0x1C00}else{0x1800};
        mapoffs += (((self.line + self.scy) as u16) & 255) >> 3;
        let lineoff = (self.scx >> 3);

        let y = (self.line + self.scy) & 7;

        let x = self.scx & 7;

        let canvasoff = self.line * 160 * 4;

        let mut color = 0;
        let tile = self.vram[(mapoffs + lineoff as u16) as usize];
        if self.bg_map == 1 && tile < 128 {tile += 256};
        for i in 0..160{

        }
    }
}

pub enum GPU_Mode{
    OAM_Read,
    VRAM_Read,
    HBlank,
    VBlank
}