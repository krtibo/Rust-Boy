/*************************************************************************

                    ===    THIS IS RUST BOY    ===

    This is the Memory Map rendering class. It renders every byte of the
    RAM as a pixel. The pixel is dark if that exact byte is not 0. If it
    is, its light colored.

    If the user clicks on one of the pixels, the print_ram function prints
    its value to the terminal.

    PARAMETERS :
        * CPU
        
*************************************************************************/

use cpu::CPU;
extern crate minifb;

pub struct MemoryMap {
    pub ram : [u32; 350*200],
    pub window : minifb::Window,
}

impl MemoryMap {

    pub fn new() -> MemoryMap {

        MemoryMap {
            ram : [0; 350*200],
            window : minifb::Window::new("RAM Map",
                         350,
                         200,
                         minifb::WindowOptions {
                             resize: false,
                             scale: minifb::Scale::X4,
                             ..minifb::WindowOptions::default()})
                             .unwrap(),
        }
    }

    pub fn print_ram(& mut self, cpu : &CPU) {

        for i in 0..65535 {

            if cpu.RAM[i] > 0 {
                //println!("{:04X} : {:02X}", i, cpu.RAM[i]);
                self.ram[i] = 0xFF0F380F;
                //print!("{:X} ", cpu.RAM[i]);
                if i >= 0xFE00 && i <= 0xFE9F { self.ram[i] = 0xFFF20056; } // sprite attribute table
                if i >= 0x8000 && i <= 0x8FFF { self.ram[i] = 0xFF003399; } // tile 1
                if i >= 0x8800 && i <= 0x97FF { self.ram[i] = 0xFF003399; } // tile 2
                if i >= 0x9800 && i <= 0x9BFF { self.ram[i] = 0xFF453823; } // window 1
                if i >= 0x9C00 && i <= 0x9FFF { self.ram[i] = 0xFF453823; } // window 2
            } else {
                self.ram[i] = 0xFF9BBC0F;
                if i >= 0xFE00 && i <= 0xFE9F { self.ram[i] = 0xFFFFBAD2; }
                if i >= 0x8000 && i <= 0x8FFF { self.ram[i] = 0xFF66CCFF; }
                if i >= 0x8800 && i <= 0x97FF { self.ram[i] = 0xFF66CCFF; }
                if i >= 0x9800 && i <= 0x9BFF { self.ram[i] = 0xFFBAA378; }
                if i >= 0x9C00 && i <= 0x9FFF { self.ram[i] = 0xFFBAA378; }
            }


        }
        self.window.update_with_buffer(&self.ram).unwrap();
        if self.window.get_mouse_down(minifb::MouseButton::Left) {
            self.window.get_mouse_pos(minifb::MouseMode::Clamp).map(|mouse| {
                if mouse.0 as u32 + mouse.1 as u32 * 350 <= 65535 {
                    let pos : u16 = mouse.0 as u16 + mouse.1 as u16 * 350;
                    println!("BYTE: 0x{:02X} 0b{:08b} -- POSITION: 0x{:02X}",
                    cpu.RAM[pos as usize],
                    cpu.RAM[pos as usize],
                    pos);
                }

            });
        }

    }
}
