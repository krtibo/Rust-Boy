use cpu::CPU;

pub struct PPU {
    scanline_count : u16,

}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            scanline_count : 0,
        }
    }

    pub fn update_n_sync(&mut self, cpu : &mut CPU, cycle : u8) {
        self.scanline_count += cycle as u16;
        if self.scanline_count >= 456 {
            cpu.RAM[0xFF44] += 1;
            
            self.scanline_count = 0;

            if cpu.RAM[0xFF44] < 144 {
                // draw line
            }

            if cpu.RAM[0xFF44] == 144 {
                // VBlank interrupt
            }

            if cpu.RAM[0xFF44] > 153 {
                cpu.RAM[0xFF44] = 0;
            }
        }
    }
}