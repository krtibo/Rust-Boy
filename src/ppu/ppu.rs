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
    pub window : Window,
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

        if PPU::get_bit(0, cpu.RAM[0xFF40]) == 1  {
            self.draw_tile(cpu);
        }

        if PPU::get_bit(1, cpu.RAM[0xFF40]) == 1  {
            self.draw_sprite(cpu);
        }
    }



    fn draw_sprite(&mut self, cpu : &mut CPU) {
        let mut double_size : bool = false;

        if PPU::get_bit(2, self.lcd_ctrl) == 1 {
            double_size = true;
        }

        for i in 0..40 {
            let sprite_addr : u16 = 0xFE00 + (39 - i) as u16 * 4;
            let y_pos : i32 = cpu.RAM[sprite_addr as usize] as u16 as i32 - 16;
            let x_pos : i32 = cpu.RAM[(sprite_addr + 1) as usize] as u16 as i32 - 8;
            let tile_location : u16 = ((cpu.RAM[(sprite_addr + 2) as usize]) &
                        (if double_size == true { 0xFE } else { 0xFF })) as u16;
            let attrib = cpu.RAM[(sprite_addr + 3) as usize];
            let flip_x : bool = attrib & (1 << 5) != 0;
            let flip_y : bool = attrib & (1 << 6) != 0;

            let current_scanline : i32 = cpu.RAM[0xFF44] as i32;
            let size_y : i32 = if double_size { 16 } else { 8 };

            if current_scanline < y_pos ||
            current_scanline >= (y_pos + size_y)  { continue }

            println!("addr {} {} {}",
            cpu.RAM[sprite_addr as usize],
            cpu.RAM[(sprite_addr + 1) as usize],
            cpu.RAM[(sprite_addr + 3) as usize]);

            let line : u16 = if flip_y {
                (size_y - 1 - (current_scanline - y_pos)) as u16
            } else {
                (current_scanline - y_pos) as u16
            };

            let data_addr = 0x8000 + (tile_location * 16) + line * 2;

            let data_1 : u8 = cpu.RAM[data_addr as usize];
            let data_2 : u8 = cpu.RAM[(data_addr + 1) as usize];

            for j in (0..7).rev() {
                let mut color_bit : i16 = j;

                if flip_x {
                    color_bit -= 7;
                    color_bit *= -1;
                }

                let mut colour_num : u8 = PPU::get_bit(color_bit as u8, data_2);
                colour_num = colour_num << 1;
                colour_num |= PPU::get_bit(color_bit as u8, data_1);

                let color_address : u16 =
                if PPU::get_bit(4, attrib) == 1 { 0xFF49 } else { 0xFF48 };
                let color : u32 = self.select_colors(cpu, colour_num, color_address);

                if color == 0xFF9BBC0F {
                    continue;
                }

                let pixel : u8 = x_pos as u8 + (7 - j as u8);

                self.framebuffer[PPU::coords((pixel, cpu.RAM[0xFF44])) as usize] =
                color;
            }
        }
    }



    fn draw_tile(&mut self, cpu : &mut CPU) {
        let mut tile_data : u16 = 0;
        let mut unsigned_data : bool = true;
        let mut bg_mem : u16 = 0;
        let mut y_pos : u8 = 0;
        let mut using_window = false;

        if PPU::get_bit(5, self.lcd_ctrl) == 1 {
        // window enabled
            if self.window_y <= cpu.RAM[0xFF44] {
            // window on current line?
                using_window = true;
            }
        }

        if PPU::get_bit(4, self.lcd_ctrl) == 1 {
        // tile data position
            tile_data = 0x8000;
        } else {
            tile_data = 0x8800;
            unsigned_data = false;
        }

        if !using_window {
            if PPU::get_bit(3, self.lcd_ctrl) == 1 {
                bg_mem = 0x9C00;
            } else {
                bg_mem = 0x9800;
            }
        } else {
            if PPU::get_bit(6, self.lcd_ctrl) == 1 {
                bg_mem = 0x9C00;
            } else {
                bg_mem = 0x9800;
            }
        }

        if !using_window {
            y_pos = self.scroll_y.wrapping_add(cpu.RAM[0xFF44]);
        } else {
            y_pos = cpu.RAM[0xFF44] - self.window_y;
        }

        let tile_row : u16 = (y_pos as u16 / 8) * 32;

        for i in 0..160 {
            let mut x_pos : u8 = i + self.scroll_x;

            if using_window && i >= self.window_x {
                x_pos = i - self.window_x;
            }

            let tile_col : u16 = x_pos as u16 / 8;
            let tile_addr : u16 = bg_mem + tile_row + tile_col;

            let tile_num_u : u8 = cpu.RAM[tile_addr as usize];
            let tile_num_i : i8 = cpu.RAM[tile_addr as usize] as i8;

            let mut tile_location : u16 = tile_data;

            if unsigned_data {
                tile_location += tile_num_u as u16 * 16;
            } else {
                tile_location += ((tile_num_i as i16 + 128) * 16) as u16;
            }

            let mut line : u8 = y_pos % 8;
            line *= 2;

            let d1 : u8 = cpu.RAM[(tile_location + line as u16) as usize];
            let d2 : u8 = cpu.RAM[(tile_location + line as u16 + 1) as usize];

            let mut colourbit : i16 = x_pos as i16 % 8;
            colourbit -= 7;
            colourbit *= -1;

            let mut colour_num : u8 = PPU::get_bit(colourbit as u8, d2);
            colour_num = colour_num << 1;
            colour_num |= PPU::get_bit(colourbit as u8, d1);

            self.framebuffer[PPU::coords((i,cpu.RAM[0xFF44])) as usize] =
            self.select_colors(cpu, colour_num, 0xFF47);

        }

    }


    fn select_colors(&mut self, cpu : &mut CPU, color_id : u8, addr : u16) -> u32 {
        let mut color_hex : u32 = 0;
        let mut color : u8 = 0;
        let palette : u8 = cpu.RAM[addr as usize];
        let mut high_bits : u8 = 0;
        let mut low_bits : u8 = 0;

        match color_id {
            0 => { low_bits = 0; high_bits = 1; },
            1 => { low_bits = 2; high_bits = 3; },
            2 => { low_bits = 4; high_bits = 5; },
            3 => { low_bits = 6; high_bits = 7; },
            _ => { low_bits = 0; high_bits = 1; }
        }

        color = PPU::get_bit(high_bits, palette) << 1;
        color = color | PPU::get_bit(low_bits, palette);

        match color {
            0 => color_hex = 0xFF9BBC0F,
            1 => color_hex = 0xFF8BAC0F,
            2 => color_hex = 0xFF306230,
            3 => color_hex = 0xFF0F380F,
            _ => color_hex = 0
        }

        color_hex
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
        (xy.1 - 1) as u16 * 160 + xy.0 as u16
    }
}
