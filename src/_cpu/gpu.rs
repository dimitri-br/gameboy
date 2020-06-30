// GPU code from - https://github.com/mvdnes/rboy/
// Used for debugging, I may use a custom system in future!
const VRAM_SIZE : usize = 0x4000;
const VOAM_SIZE : usize = 0xA0;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;



#[derive(PartialEq, Copy, Clone)]
enum PrioType {
    Color0,
    PrioFlag,
    Normal,
}

pub struct GPU{
    mode: u8,
    modeclock: u32,
    pub line: u8,
    lyc: u8,
    lcd_on: bool,
    win_tilemap: u16,
    win_on: bool,
    tilebase: u16,
    bg_tilemap: u16,
    sprite_size: u32,
    sprite_on: bool,
    lcdc0: bool,
    lyc_inte: bool,
    m0_inte: bool,
    m1_inte: bool,
    m2_inte: bool,
    scx: u8,
    scy: u8,
    winx: u8,
    winy: u8,
    palbr: u8,
    pal0r: u8,
    pal1r: u8,
    palb: [u8; 4],
    pal0: [u8; 4],
    pal1: [u8; 4],

    vram: [u8; VRAM_SIZE],
    voam: [u8; VOAM_SIZE],
    cbgpal_inc: bool,
    cbgpal_ind: u8,
    cbgpal: [[[u8; 3]; 4]; 8],
    csprit_inc: bool,
    csprit_ind: u8,
    csprit: [[[u8; 3]; 4]; 8],
    vrambank: usize,
    pub data: Vec<u8>,
    bgprio: [PrioType; SCREEN_W],
    pub updated: bool,
    pub interrupt: u8,

    hblanking: bool,

}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            mode: 0,
            modeclock: 0,
            line: 0,
            lyc: 0,
            lcd_on: false,
            win_tilemap: 0x9C00,
            win_on: false,
            tilebase: 0x8000,
            bg_tilemap: 0x9C00,
            sprite_size: 8,
            sprite_on: false,
            lcdc0: false,
            lyc_inte: false,
            m2_inte: false,
            m1_inte: false,
            m0_inte: false,
            scy: 0,
            scx: 0,
            winy: 0,
            winx: 0,
            palbr: 0,
            pal0r: 0,
            pal1r: 1,
            palb: [0; 4],
            pal0: [0; 4],
            pal1: [0; 4],
            vram: [0; VRAM_SIZE],
            voam: [0; VOAM_SIZE],
            data: vec![0; SCREEN_W * SCREEN_H * 3],
            bgprio: [PrioType::Normal; SCREEN_W],
            updated: false,
            interrupt: 0,
            cbgpal_inc: false,
            cbgpal_ind: 0,
            cbgpal: [[[0u8; 3]; 4]; 8],
            csprit_inc: false,
            csprit_ind: 0,
            csprit: [[[0u8; 3]; 4]; 8],
            vrambank: 0,
            hblanking: false,
        }
    }
}
impl GPU {
    pub fn new_cgb() -> GPU {
        GPU::new()
    }

    pub fn do_cycle(&mut self, ticks: u32) {
        if !self.lcd_on { return }
        self.hblanking = false;

        let mut ticksleft = ticks;

        while ticksleft > 0 {
            let curticks = if ticksleft >= 80 { 80 } else { ticksleft };
            self.modeclock += curticks;
            ticksleft -= curticks;

            // Full line takes 114 ticks
            if self.modeclock >= 456 {
                self.modeclock -= 456;
                self.line = (self.line + 1) % 154;
                self.check_interrupt_lyc();

                // This is a VBlank line
                if self.line >= 144 && self.mode != 1 {
                    self.change_mode(1);
                }
            }

            // This is a normal line
            if self.line < 144 {
                if self.modeclock <= 80 {
                    if self.mode != 2 { self.change_mode(2); }
                } else if self.modeclock <= (80 + 172) { // 252 cycles
                    if self.mode != 3 { self.change_mode(3); }
                } else { // the remaining 204
                    if self.mode != 0 { self.change_mode(0); }
                }
            }
        }
    }

    fn check_interrupt_lyc(&mut self) {
        if self.lyc_inte && self.line == self.lyc {
            self.interrupt |= 0x02;
        }
    }

    fn change_mode(&mut self, mode: u8) {
        self.mode = mode;

        if match self.mode {
            0 => {
                self.renderscan();
                self.hblanking = true;
                self.m0_inte
            },
            1 => {
                self.interrupt |= 0x01;
                self.updated = true;
                self.m1_inte
            },
            2 => self.m2_inte,
            _ => false,
        } {
            self.interrupt |= 0x02;
        }
    }
}
impl GPU {
    pub fn rb(&mut self, address: u16) -> u8{
        match address{
            0x8000 ..= 0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00 ..= 0xFE9F => self.voam[address as usize - 0xFE00],
            0xFF40 => {
                (if self.lcd_on { 0x80 } else { 0 }) |
                (if self.win_tilemap == 0x9C00 { 0x40 } else { 0 }) |
                (if self.win_on { 0x20 } else { 0 }) |
                (if self.tilebase == 0x8000 { 0x10 } else { 0 }) |
                (if self.bg_tilemap == 0x9C00 { 0x08 } else { 0 }) |
                (if self.sprite_size == 16 { 0x04 } else { 0 }) |
                (if self.sprite_on { 0x02 } else { 0 }) |
                (if self.lcdc0 { 0x01 } else { 0 })
            },
            0xFF41 => {
                (if self.lyc_inte { 0x40 } else { 0 }) |
                (if self.m2_inte { 0x20 } else { 0 }) |
                (if self.m1_inte { 0x10 } else { 0 }) |
                (if self.m0_inte { 0x08 } else { 0 }) |
                (if self.line == self.lyc { 0x04 } else { 0 }) |
                self.mode
            },
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF45 => self.lyc,
            0xFF46 => 0, // Write only
            0xFF47 => self.palbr,
            0xFF48 => self.pal0r,
            0xFF49 => self.pal1r,
            0xFF4A => self.winy,
            0xFF4B => self.winx,
            0xFF4F => self.vrambank as u8,
            0xFF68 => { self.cbgpal_ind | (if self.cbgpal_inc { 0x80 } else { 0 }) },
            0xFF69 => {
                let palnum = (self.cbgpal_ind >> 3) as usize;
                let colnum = ((self.cbgpal_ind >> 1) & 0x3) as usize;
                if self.cbgpal_ind & 0x01 == 0x00 {
                    self.cbgpal[palnum][colnum][0] | ((self.cbgpal[palnum][colnum][1] & 0x07) << 5)
                } else {
                    ((self.cbgpal[palnum][colnum][1] & 0x18) >> 3) | (self.cbgpal[palnum][colnum][2] << 2)
                }
            },
            0xFF6A => { self.csprit_ind | (if self.csprit_inc { 0x80 } else { 0 }) },
            0xFF6B => {
                let palnum = (self.csprit_ind >> 3) as usize;
                let colnum = ((self.csprit_ind >> 1) & 0x3) as usize;
                if self.csprit_ind & 0x01 == 0x00 {
                    self.csprit[palnum][colnum][0] | ((self.csprit[palnum][colnum][1] & 0x07) << 5)
                } else {
                    ((self.csprit[palnum][colnum][1] & 0x18) >> 3) | (self.csprit[palnum][colnum][2] << 2)
                }
            },
            _ => panic!("GPU does not handle read {:04X}", address),
        }
    }
    fn rbvram0(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 { panic!("Shouldn't have used rbvram0"); }
        self.vram[a as usize & 0x1FFF]
    }
    fn rbvram1(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 { panic!("Shouldn't have used rbvram1"); }
        self.vram[0x2000 + (a as usize & 0x1FFF)]
    }
    pub fn wb(&mut self, address: u16, value: u8) {
        match address {
            0x8000 ..= 0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)] = value,
            0xFE00 ..= 0xFE9F => self.voam[address as usize - 0xFE00] = value,
            0xFF40 => {
                let orig_lcd_on = self.lcd_on;
                self.lcd_on = value & 0x80 == 0x80;
                self.win_tilemap = if value & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
                self.win_on = value & 0x20 == 0x20;
                self.tilebase = if value & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
                self.bg_tilemap = if value & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
                self.sprite_size = if value & 0x04 == 0x04 { 16 } else { 8 };
                self.sprite_on = value & 0x02 == 0x02;
                self.lcdc0 = value & 0x01 == 0x01;
                if orig_lcd_on && !self.lcd_on { self.modeclock = 0; self.line = 0; self.mode = 0; self.clear_screen(); }
            },
            0xFF41 => {
                self.lyc_inte = value & 0x40 == 0x40;
                self.m2_inte = value & 0x20 == 0x20;
                self.m1_inte = value & 0x10 == 0x10;
                self.m0_inte = value & 0x08 == 0x08;
            },
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {}, // Read-only
            0xFF45 => self.lyc = value,
            0xFF46 => panic!("0xFF46 should be handled by MMU"),
            0xFF47 => { self.palbr = value; self.update_pal(); },
            0xFF48 => { self.pal0r = value; self.update_pal(); },
            0xFF49 => { self.pal1r = value; self.update_pal(); },
            0xFF4A => self.winy = value,
            0xFF4B => self.winx = value,
            0xFF4F => self.vrambank = (value & 0x01) as usize,
            0xFF68 => { self.cbgpal_ind = value & 0x3F; self.cbgpal_inc = value & 0x80 == 0x80; },
            0xFF69 => {
                let palnum = (self.cbgpal_ind >> 3) as usize;
                let colnum = ((self.cbgpal_ind >> 1) & 0x03) as usize;
                if self.cbgpal_ind & 0x01 == 0x00 {
                    self.cbgpal[palnum][colnum][0] = value & 0x1F;
                    self.cbgpal[palnum][colnum][1] = (self.cbgpal[palnum][colnum][1] & 0x18) | (value >> 5);
                } else {
                    self.cbgpal[palnum][colnum][1] = (self.cbgpal[palnum][colnum][1] & 0x07) | ((value & 0x3) << 3);
                    self.cbgpal[palnum][colnum][2] = (value >> 2) & 0x1F;
                }
                if self.cbgpal_inc { self.cbgpal_ind = (self.cbgpal_ind + 1) & 0x3F; };
            },
            0xFF6A => { self.csprit_ind = value & 0x3F; self.csprit_inc = value & 0x80 == 0x80; },
            0xFF6B => {
                let palnum = (self.csprit_ind >> 3) as usize;
                let colnum = ((self.csprit_ind >> 1) & 0x03) as usize;
                if self.csprit_ind & 0x01 == 0x00 {
                    self.csprit[palnum][colnum][0] = value & 0x1F;
                    self.csprit[palnum][colnum][1] = (self.csprit[palnum][colnum][1] & 0x18) | (value >> 5);
                } else {
                    self.csprit[palnum][colnum][1] = (self.csprit[palnum][colnum][1] & 0x07) | ((value & 0x3) << 3);
                    self.csprit[palnum][colnum][2] = (value >> 2) & 0x1F;
                }
                if self.csprit_inc { self.csprit_ind = (self.csprit_ind + 1) & 0x3F; };
            },
            _ => {},
        }
    }
}
impl GPU {
    fn clear_screen(&mut self) {
        for v in self.data.iter_mut() {
            *v = 255;
        }
        self.updated = true;
    }

    fn update_pal(&mut self) {
        for i in 0 .. 4 {
            self.palb[i] = GPU::get_monochrome_pal_val(self.palbr, i);
            self.pal0[i] = GPU::get_monochrome_pal_val(self.pal0r, i);
            self.pal1[i] = GPU::get_monochrome_pal_val(self.pal1r, i);
        }
    }

    fn get_monochrome_pal_val(value: u8, index: usize) -> u8 {
        match (value >> 2*index) & 0x03 {
            0 => 255,
            1 => 192,
            2 => 96,
            _ => 0
        }
    }

    fn renderscan(&mut self) {
        for x in 0 .. SCREEN_W {
            self.setcolor(x, 255);
            self.bgprio[x] = PrioType::Normal;
        }
        self.draw_bg();
        self.draw_sprites();
    }

    fn setcolor(&mut self, x: usize, color: u8) {
        self.data[self.line as usize * SCREEN_W * 3 + x * 3 + 0] = color;
        self.data[self.line as usize * SCREEN_W * 3 + x * 3 + 1] = color;
        self.data[self.line as usize * SCREEN_W * 3 + x * 3 + 2] = color;
    }

    fn setrgb(&mut self, x: usize, r: u8, g: u8, b: u8) {
        // Gameboy Color RGB correction
        // Taken from the Gambatte emulator
        // assume r, g and b are between 0 and 1F
        let baseidx = self.line as usize * SCREEN_W * 3 + x * 3;

        let r = r as u32;
        let g = g as u32;
        let b = b as u32;

        self.data[baseidx + 0] = ((r * 13 + g * 2 + b) >> 1) as u8;
        self.data[baseidx + 1] = ((g * 3 + b) << 1) as u8;
        self.data[baseidx + 2] = ((r * 3 + g * 2 + b * 11) >> 1) as u8;
    }

    fn draw_bg(&mut self) {
        let drawbg = self.lcdc0;

        let winy =
            if !self.win_on  { -1 }
            else { self.line as i32 - self.winy as i32 };

        if winy < 0 && drawbg == false {
            return;
        }

        let wintiley = (winy as u16 >> 3) & 31;

        let bgy = self.scy.wrapping_add(self.line);
        let bgtiley = (bgy as u16 >> 3) & 31;

        for x in 0 .. SCREEN_W {
            let winx = - ((self.winx as i32) - 7) + (x as i32);
            let bgx = self.scx as u32 + x as u32;

            let (tilemapbase, tiley, tilex, pixely, pixelx) = if winy >= 0 && winx >= 0 {
                (self.win_tilemap,
                wintiley,
                (winx as u16 >> 3),
                winy as u16 & 0x07,
                winx as u8 & 0x07)
            } else if drawbg {
                (self.bg_tilemap,
                bgtiley,
                (bgx as u16 >> 3) & 31,
                bgy as u16 & 0x07,
                bgx as u8 & 0x07)
            } else {
                continue;
            };

            let tilenr: u8 = self.rbvram0(tilemapbase + tiley * 32 + tilex);

            let (palnr, vram1, xflip, yflip, prio) = if false {
                let flags = self.rbvram1(tilemapbase + tiley * 32 + tilex) as usize;
                (flags & 0x07,
                flags & (1 << 3) != 0,
                flags & (1 << 5) != 0,
                flags & (1 << 6) != 0,
                flags & (1 << 7) != 0)
            } else {
                (0, false, false, false, false)
            };

            let tileaddress = self.tilebase
            + (if self.tilebase == 0x8000 {
                tilenr as u16
            } else {
                (tilenr as i8 as i16 + 128) as u16
            }) * 16;

            let a0 = match yflip {
                false => tileaddress + (pixely * 2),
                true => tileaddress + (14 - (pixely * 2)),
            };

            let (b1, b2) = match vram1 {
                false => (self.rbvram0(a0), self.rbvram0(a0 + 1)),
                true => (self.rbvram1(a0), self.rbvram1(a0 + 1)),
            };

            let xbit = match xflip {
                true => pixelx,
                false => 7 - pixelx,
            } as u32;
            let colnr = if b1 & (1 << xbit) != 0 { 1 } else { 0 }
                | if b2 & (1 << xbit) != 0 { 2 } else { 0 };

            self.bgprio[x] =
                if colnr == 0 { PrioType::Color0 }
                else if prio { PrioType::PrioFlag }
                else { PrioType::Normal };
            if false {
                let r = self.cbgpal[palnr][colnr][0];
                let g = self.cbgpal[palnr][colnr][1];
                let b = self.cbgpal[palnr][colnr][2];
                self.setrgb(x as usize, r, g, b);
            } else {
                let color = self.palb[colnr];
                self.setcolor(x, color);
            }
        }
    }

    fn draw_sprites(&mut self) {
        if !self.sprite_on { return }

        // TODO: limit of 10 sprites per line

        for index in 0 .. 40 {
            let i = 39 - index;
            let spriteaddr = 0xFE00 + (i as u16) * 4;
            let spritey = self.rb(spriteaddr + 0) as u16 as i32 - 16;
            let spritex = self.rb(spriteaddr + 1) as u16 as i32 - 8;
            let tilenum = (self.rb(spriteaddr + 2) & (if self.sprite_size == 16 { 0xFE } else { 0xFF })) as u16;
            let flags = self.rb(spriteaddr + 3) as usize;
            let usepal1: bool = flags & (1 << 4) != 0;
            let xflip: bool = flags & (1 << 5) != 0;
            let yflip: bool = flags & (1 << 6) != 0;
            let belowbg: bool = flags & (1 << 7) != 0;
            let c_palnr = flags & 0x07;
            let c_vram1: bool = flags & (1 << 3) != 0;

            let line = self.line as i32;
            let sprite_size = self.sprite_size as i32;

            if line < spritey || line >= spritey + sprite_size { continue }
            if spritex < -7 || spritex >= (SCREEN_W as i32) { continue }

            let tiley: u16 = if yflip {
                (sprite_size - 1 - (line - spritey)) as u16
            } else {
                (line - spritey) as u16
            };

            let tileaddress = 0x8000u16 + tilenum * 16 + tiley * 2;
            let (b1, b2) = if c_vram1 && false {
                (self.rbvram1(tileaddress), self.rbvram1(tileaddress + 1))
            } else {
                (self.rbvram0(tileaddress), self.rbvram0(tileaddress + 1))
            };

            'xloop: for x in 0 .. 8 {
                if spritex + x < 0 || spritex + x >= (SCREEN_W as i32) { continue }

                let xbit = 1 << (if xflip { x } else { 7 - x } as u32);
                let colnr = (if b1 & xbit != 0 { 1 } else { 0 }) |
                    (if b2 & xbit != 0 { 2 } else { 0 });
                if colnr == 0 { continue }

                if false {
                    if self.lcdc0 && (self.bgprio[(spritex + x) as usize] == PrioType::PrioFlag || (belowbg && self.bgprio[(spritex + x) as usize] != PrioType::Color0)) {
                        continue 'xloop
                    }
                    let r = self.csprit[c_palnr][colnr][0];
                    let g = self.csprit[c_palnr][colnr][1];
                    let b = self.csprit[c_palnr][colnr][2];
                    self.setrgb((spritex + x) as usize, r, g, b);
                } else {
                    if belowbg && self.bgprio[(spritex + x) as usize] != PrioType::Color0 { continue 'xloop }
                    let color = if usepal1 { self.pal1[colnr] } else { self.pal0[colnr] };
                    self.setcolor((spritex + x) as usize, color);
                }
            }
        }
    }

    pub fn may_hdma(&self) -> bool {
        return self.hblanking;

    }
}