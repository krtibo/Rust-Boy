#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;
extern crate minifb;

use self::minifb::{WindowOptions, Window, Scale};

pub struct PPU {
    scanline_count : u16,
    lcd_ctrl : u8,
    scroll_x : u8,
    scroll_y : u8,
    window_x : u8,
    window_y : u8,
    window : Window,
    framebuffer : [u32; 160*144],

}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            scanline_count : 0,
            lcd_ctrl : 0,
            scroll_x : 0,
            scroll_y : 0,
            window_x : 0,
            window_y : 0,
            window : Window::new("RUST BOY",
                         160,
                         144,
                         WindowOptions {
                             resize: false,
                             scale: Scale::X2,
                             ..WindowOptions::default()})
                             .unwrap(),
            framebuffer : [0; 160*144],
        }
    }

    pub fn update_n_sync(&mut self, cpu : &mut CPU, cycle : u8) {
        self.lcd_ctrl = cpu.RAM[0xFF40];
        self.scroll_x = cpu.RAM[0xFF43];
        self.scroll_y = cpu.RAM[0xFF42];
        self.window_x = cpu.RAM[0xFF4B];
        self.window_y = cpu.RAM[0xFF4A];

        self.scanline_count += cycle as u16;
        if self.scanline_count >= 456 {
            cpu.RAM[0xFF44] += 1;
            
            self.scanline_count = 0;

            if cpu.RAM[0xFF44] < 144 {
                // draw line
                self.draw_line(cpu);
            }

            if cpu.RAM[0xFF44] == 144 {
                cpu.IRQ(0);
            }

            if cpu.RAM[0xFF44] > 153 {
                cpu.RAM[0xFF44] = 0;
            }
        }
    }


    fn draw_line(&mut self, cpu : &mut CPU) {
        let tile_data : u16 = 0x8000;
        let bg_mem : u16 = 0x9800;
        let mut y_pos : u8 = 0;

        y_pos = self.scroll_y + cpu.RAM[0xFF44];
        let tile_row : u16 = (y_pos as u16 / 8) * 32;

        for i in 0..160 {
            let x_pos : u8 = i + self.scroll_x;
            let tile_col : u16 = x_pos as u16 / 8;
            let tile_addr : u16 = bg_mem + tile_row + tile_col; 
            let tile_num : u8 = cpu.RAM[tile_addr as usize];

            let mut tile_location : u16 = tile_data;
            tile_location += tile_num as u16 * 16;
            let mut line : u8 = y_pos % 8;
            line *= 2;

            let d1 : u8 = cpu.RAM[(tile_location + line as u16) as usize];
            let d2 : u8 = cpu.RAM[(tile_location + line as u16 + 1) as usize];
            //println!("{:02X} {:02X}", tile_location + line as u16, tile_location + line as u16 + 1);

            let mut colourbit : i16 = x_pos as i16 % 8;
            colourbit -= 7;
            colourbit *= -1;

            let mut colour_num : u8 = PPU::get_bit(colourbit as u8, d2);
            colour_num = colour_num << 1;
            colour_num |= PPU::get_bit(colourbit as u8, d1);

            //let check : u8 = cpu.RAM[0xFF44];

            if colour_num > 0 {
                self.framebuffer[PPU::coords((i,cpu.RAM[0xFF44])) as usize] = 0xFF_00_FF_00;
            }  else {
                self.framebuffer[PPU::coords((i,cpu.RAM[0xFF44])) as usize] = 0;
            }
                
        }
        
    }

    fn lcd_status(&self, cpu : &mut CPU) -> bool {
    // get the state of the LCD control register

        if PPU::get_bit(7, cpu.RAM[0xFF40]) == 1 {
            true
        } else {
            false
        }
    }

    fn lcd_status_update(&mut self, cpu : &mut CPU) {

        let mut lcd_status_reg : u8 = cpu.RAM[0xFF41];

        if !self.lcd_status(cpu) {
            self.scanline_count = 456;
            cpu.RAM[0xFF44] = 0;
            lcd_status_reg = lcd_status_reg & 0b1111_1100;
            lcd_status_reg = PPU::set_bit(0, lcd_status_reg);
            cpu.write_ram(0xFF41, lcd_status_reg);
            return
        }

        let current_scanline : u8 = cpu.RAM[0xFF44];
        let lcd_current_mode : u8 = lcd_status_reg & 0b0000_0011;
        let mut lcd_mode : u8 = 0;
        let mut IRQ : bool = false;

        if current_scanline >= 144 {
        // let us handle mode 1 interrupt

            lcd_mode = 1;
            lcd_status_reg = PPU::set_bit(0, lcd_status_reg);
            lcd_status_reg = PPU::reset_bit(1, lcd_status_reg);
            IRQ = if PPU::get_bit(4, lcd_status_reg) == 1 { true } else { false };

        } else {

            let lcd_mode_2_lim : u16 = 456 - 80;
            let lcd_mode_3_lim : u16 = lcd_mode_2_lim - 172;
            
            if self.scanline_count >= lcd_mode_2_lim {
            // its time for handling mode 2 interrupt

                lcd_mode = 2;
                lcd_status_reg = PPU::set_bit(1, lcd_status_reg);
                lcd_status_reg = PPU::reset_bit(0, lcd_status_reg);
                IRQ = if PPU::get_bit(5, lcd_status_reg) == 1 { true } else { false };
            
            } else if self.scanline_count >= lcd_mode_3_lim {
            // and now the mode 3 interrupt

                lcd_mode = 3;
                lcd_status_reg = PPU::set_bit(1, lcd_status_reg);
                lcd_status_reg = PPU::set_bit(0, lcd_status_reg);

            } else {
            // the last one is the mode 0 interrupt

                lcd_mode = 0;
                lcd_status_reg = PPU::reset_bit(1, lcd_status_reg);
                lcd_status_reg = PPU::reset_bit(0, lcd_status_reg);
                IRQ = if PPU::get_bit(3, lcd_status_reg) == 1 { true } else { false };
            }
        }

        if IRQ == true && lcd_mode != lcd_current_mode {
        // IRQ time!

            cpu.IRQ(1);
        }

        if cpu.RAM[0xFF44] == cpu.RAM[0xFF45] {
            lcd_status_reg = PPU::set_bit(2, lcd_status_reg);

            if PPU::get_bit(6, lcd_status_reg) == 1 {
                cpu.IRQ(1);
            }
        } else {
            lcd_status_reg = PPU::reset_bit(2, lcd_status_reg);
        }

        cpu.write_ram(0xFF41, lcd_status_reg);
    }


    fn get_bit(n : u8, reg : u8) -> u8 {
        let mask : u8 = 1 << n;
        
        if reg & mask == 0 {
            0
        } else {
            1
        }
    }


    fn set_bit(n : u8, reg : u8) -> u8 {
        let mut value : u8 = 0;
        let mask : u8 = 1 << n;
        value = reg | mask;
        value
    }


    fn reset_bit(n : u8, reg : u8) -> u8 {
        let mut value : u8 = 0;
        let mask : u8 = 1 << n;
        value = reg & (0xFF - mask);
        value
    }


    pub fn render(&mut self) {
        self.window.update_with_buffer(&self.framebuffer).unwrap();
    }

    fn coords(xy : (u8, u8)) -> u16 {
        xy.1 as u16 * 160 + xy.0 as u16
    }
}