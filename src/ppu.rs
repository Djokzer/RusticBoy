pub struct PPU 
{
	mode: u8, 			//Mode 0: HBlank, 1: VBlank, 2: OAM Scan, 3: Drawing Pixels
	mode_cycle : u32,	//Cycle in the current mode
	ly: u8, 			//Current line
	lyc: u8, 			//LYC register: if LY == LYC, then an interrupt is triggered
	stat: u8, 			//STAT register

	framebuffer: [u8; 160 * 144 * 4],	//Framebuffer for the current frame
	scanline: [u8; 160 * 4], 			//Scanline buffer for the current line
}

impl PPU {
    pub fn init_ppu() -> Self {
        PPU {
            mode: 2,    
            mode_cycle: 0,
            ly: 0,
            lyc: 0,
            stat: 0,
            framebuffer: [0; 160 * 144 * 4],
            scanline: [0; 160 * 4],        
        }
    }

	pub fn step(&mut self, cycles: u32) {
        self.mode_cycle += cycles;

        match self.mode {
            2 => { // OAM Scan
                if self.mode_cycle >= 80 {
                    self.mode_cycle = 0;
                    self.mode = 3;
                }
            }
            3 => { // Drawing pixels
                if self.mode_cycle >= 172 {
                    self.mode_cycle = 0;
                    self.mode = 0;
                    self.copy_scanline();
                }
            }
            0 => { // HBlank
                if self.mode_cycle >= 204 {
                    self.mode_cycle = 0;
                    self.ly += 1;

                    if self.ly == 144 {
                        self.mode = 1;   // Enter VBlank
                        // ! TODO: request VBlank interrupt here
                    } else {
                        self.mode = 2;  
                    }
                }
            }
            1 => { // VBlank
                if self.mode_cycle >= 456 {
                    self.mode_cycle = 0;
                    self.ly += 1;

                    if self.ly > 153 {
                        self.ly = 0;
                        self.mode = 2;
                    }
                }
            }
            _ => ()
        }
    }

    fn copy_scanline(&mut self) {
        let start = self.ly as usize * 160 * 4;
        self.framebuffer[start..start + 160 * 4]
            .copy_from_slice(&self.scanline);
    }
}