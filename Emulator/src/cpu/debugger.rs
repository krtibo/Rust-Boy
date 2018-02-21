/*************************************************************************

                    ===    THIS IS RUST BOY    ===

    This is the instructions and registers renderer. I've written this
    program as a debugger tool for the Z80 disassembler for my thesis
    project.

    Its only function is rendering the finished instructions (which were
    done by the Z80 CPU) and the register states on every one of them.

    INPUT : Intructions (String) and register data (hex numbers in String).
            They need to have the same size! (max 50 - 50)
    OUTPUT : 256 x 360 screen buffer with the rendered data

***************************************************************************/

#![allow(unused_variables)]

extern crate minifb;
use self::minifb::{WindowOptions, Window, Scale};
use std::collections::LinkedList;

pub struct Colors {
    pub pink : u32,
    pub red : u32,
    pub blue : u32,
    pub orange : u32,
    pub green : u32,
    pub teal : u32,
    pub yellow : u32,
    pub white : u32,
    pub black : u32,

}

impl Colors {
    pub fn new() -> Colors {
        Colors {
            red : 0xFF_FF_66_66,
            pink : 0xFF_FF_93_C8,
            blue : 0xFF_CB_C8_FD,
            teal : 0xFF_71_FF_FF,
            orange : 0xFF_FF_CB_99,
            green : 0xFF_C6_FF_CD,
            yellow : 0xFF_FF_FF_9D,
            white : 0xFF_40_20_20,
            black : 0x00_00_00_00,
        }
    }
}

pub struct DebugData {
    pub instructions : LinkedList<String>,
    pub register_states : LinkedList<Vec<String>>,
}

impl DebugData {
    pub fn new() -> DebugData {
        DebugData {
            instructions : LinkedList::new(),
            register_states : LinkedList::new(),
        }
    }

    pub fn parse_data_from_cpu(&mut self, data : (String, Vec<String>)) {
        let (inst, reg) = data;

        if self.instructions.len() == 50 {
            self.instructions.pop_back();
            self.register_states.pop_back();
        }

        if self.instructions.len() < 50 {
            self.instructions.push_front(inst);
            self.register_states.push_front(reg);
        }
    }
}


pub struct Debugger {
    pub vram : [u32; 256*360],
    pub instructions : LinkedList<String>,
    pub register_states : LinkedList<Vec<String>>,
    pub print_color : u32,
    pub textbg_color : u32,
    pub colors : Colors,
    pub active_state : u8,
    pub window : Window,
}

impl Debugger {

    pub fn new() -> Debugger {

        Debugger {
            vram : [0; 256*360],
            instructions : LinkedList::new(),
            register_states : LinkedList::new(),
            print_color : 0xFF_FF_FF_FF,
            textbg_color : 0x00_00_00_00,
            colors : Colors::new(),
            active_state : 0,
            window : Window::new("RUST BOY - DISASSEMBLER",
                         256,
                         360,
                         WindowOptions {
                             resize: false,
                             scale: Scale::X2,
                             ..WindowOptions::default()})
                             .unwrap(),
        }
    }



    pub fn update_window(&mut self, dd : &DebugData){

        self.window.get_scroll_wheel().map(|scroll| {
            if scroll.1 < 0.0 && self.active_state < (&dd.instructions.len()-1) as u8 {
                self.active_state += -(scroll.1 / 10.0) as u8;
            }

            if scroll.1 > 0.0 && self.active_state > 0 {
                self.active_state -= (scroll.1 / 10.0) as u8;
            }
        });



        self.render_multiline_text(5,5,&dd.instructions);
        self.render_registers(&dd.register_states);
        self.render_line();
        self.window.update_with_buffer(&self.vram).unwrap();
    }

    pub fn render_line(&mut self) {
        for i in 5..355 {
            self.vram[i * 256 + 164] = self.colors.white;
        }

        // 172, 108

        for i in 170..251 {
            self.vram[108 * 256 + i] = self.colors.white;
        }
    }

    pub fn render_multiline_text(&mut self, x : u32, y : u32, v : &LinkedList<String>) {
        let mut index = 0;
        for i in v {

            if i.contains("NOP") || i.contains("STOP")
            || i.contains("HALT") || i.contains("DI")
            || i.contains("CB") || i.contains("EI"){
                self.print_color = self.colors.pink;
            } else

            if i.contains("LD") {
                self.print_color = self.colors.blue;
            } else

            if i.contains("JR") || i.contains("RST")
            || i.contains("RET") || i.contains("CALL"){
                self.print_color = self.colors.orange;
            } else

            if i.contains("RL") || i.contains("RR") {
                self.print_color = self.colors.teal;
            } else

            if i.contains("POP") || i.contains("PUSH") {
                self.print_color = self.colors.green;
            } else

            if i.contains("INC") || i.contains("DEC")
            || i.contains("CPL") || i.contains("CCF")
            || i.contains("ADD") || i.contains("ADC")
            || i.contains("SUB") || i.contains("SBC")
            || i.contains("AND") || i.contains("XOR")
            || i.contains("OR") || i.contains("CP")
            || i.contains("DEC"){
                self.print_color = self.colors.yellow;
            } else { self.print_color = self.colors.teal; }


            if index == self.active_state as usize {
                self.print_color = self.colors.red;
                self.textbg_color = self.colors.white;
            }

            // render the given text - if its longer than 38 chars,
            // throw the remaining chars
            if &i.len() > &37 {
                self.render_text(x, y + index as u32 *7, &i[..37].to_string());
            } else {
                self.render_text(x, y + index as u32 *7, &i);
            }

            self.print_color = 0xFF_FF_FF_FF;
            self.textbg_color = self.colors.black;
            index += 1;
            //self.print_color = rand::thread_rng().gen::<u32>();
        }
    } // fn render_multiline_text


    pub fn render_text(&mut self, x : u32, y : u32, text : &String) {

        let caps_abc : [u16;32] = [
            0b1011011111010100,     // A
            0b0111010111010110,     // B -
            0b1100010010011100,     // C
            0b0111011011010110,     // D
            0b1110011110011110,     // E
            0b0010011110011110,     // F
            0b1101011110011100,     // G
            0b1011011111011010,     // H
            0b1110100100101110,     // I
            0b0101011001001000,     // J
            0b1011010111011010,     // K
            0b1110010010010010,     // L
            0b1011011111111010,     // M
            0b1011111111111010,     // N
            0b0101011011010100,     // O
            0b0010010111010110,     // P
            0b1101111011010100,     // Q
            0b1010111111010110,     // R
            0b0111000100011100,     // S
            0b0100100100101110,     // T
            0b1101011011011010,     // U
            0b0100101011011010,     // V
            0b1011111111011010,     // W
            0b1011010101011010,     // X
            0b0100100101011010,     // Y
            0b1110010101001110,     // Z
            0b0000001110000000,     // -
            0b0000000000000000,     // space
            0b0010100000000000,     // ,
            0b1000100100101000,     // (
            0b0010100100100010,     // )
            0b0000100000100000,     // :
        ];

        let abc : [u16; 26] = [
            0b1111011100000000,     // a
            0b0111010110010010,     // b
            0b1100011100000000,     // c
            0b1101011101001000,     // d
            0b1100111011100000,     // e
            0b0100101110101000,     // f
            0b0111001111100000,     // g
            0b1011010110010010,     // h
            0b0100100100000100,     // i
            0b0111001000001000,     // j
            0b1010111010010000,     // k
            0b1110100100100110,     // l
            0b1011111110000000,     // m
            0b1011010110000000,     // n
            0b0101010100000000,     // o
            0b0010111010100000,     // p
            0b1001101010100000,     // q
            0b0010011110010000,     // r
            0b0111100111100000,     // s
            0b0100100101110100,     // t
            0b1101011010000000,     // u
            0b0101111010000000,     // v
            0b1111111010000000,     // w
            0b1010101010000000,     // x
            0b0111001101010000,     // y
            0b1110111101110000     // z
        ];




        let numbers : [u16; 10] = [
            0b1111011011011110,     // 0
            0b0100100100100100,     // 1
            0b1110011111001110,     // 2
            0b1111001101001110,     // 3
            0b1001001111011010,     // 4
            0b1111001110011110,     // 5
            0b1111011110011110,     // 6
            0b1001001001001110,     // 7
            0b1111011111011110,     // 8
            0b1111001111011110      // 9
        ];



        for i in 0..text.len() {
            match text.chars().nth(i).unwrap() {
                'A' => self.render_char(x + i as u32 *4 , y, caps_abc[0]),
                'B' => self.render_char(x + i as u32 *4 , y, caps_abc[1]),
                'C' => self.render_char(x + i as u32 *4 , y, caps_abc[2]),
                'D' => self.render_char(x + i as u32 *4 , y, caps_abc[3]),
                'E' => self.render_char(x + i as u32 *4 , y, caps_abc[4]),
                'F' => self.render_char(x + i as u32 *4 , y, caps_abc[5]),
                'G' => self.render_char(x + i as u32 *4 , y, caps_abc[6]),
                'H' => self.render_char(x + i as u32 *4 , y, caps_abc[7]),
                'I' => self.render_char(x + i as u32 *4 , y, caps_abc[8]),
                'J' => self.render_char(x + i as u32 *4 , y, caps_abc[9]),
                'K' => self.render_char(x + i as u32 *4 , y, caps_abc[10]),
                'L' => self.render_char(x + i as u32 *4 , y, caps_abc[11]),
                'M' => self.render_char(x + i as u32 *4 , y, caps_abc[12]),
                'N' => self.render_char(x + i as u32 *4 , y, caps_abc[13]),
                'O' => self.render_char(x + i as u32 *4 , y, caps_abc[14]),
                'P' => self.render_char(x + i as u32 *4 , y, caps_abc[15]),
                'Q' => self.render_char(x + i as u32 *4 , y, caps_abc[16]),
                'R' => self.render_char(x + i as u32 *4 , y, caps_abc[17]),
                'S' => self.render_char(x + i as u32 *4 , y, caps_abc[18]),
                'T' => self.render_char(x + i as u32 *4 , y, caps_abc[19]),
                'U' => self.render_char(x + i as u32 *4 , y, caps_abc[20]),
                'V' => self.render_char(x + i as u32 *4 , y, caps_abc[21]),
                'W' => self.render_char(x + i as u32 *4 , y, caps_abc[22]),
                'X' => self.render_char(x + i as u32 *4 , y, caps_abc[23]),
                'Y' => self.render_char(x + i as u32 *4 , y, caps_abc[24]),
                'Z' => self.render_char(x + i as u32 *4 , y, caps_abc[25]),
                '-' => self.render_char(x + i as u32 *4 , y, caps_abc[26]),
                ' ' => self.render_char(x + i as u32 *4 , y, caps_abc[27]),
                ',' => self.render_char(x + i as u32 *4 , y, caps_abc[28]),
                '(' => self.render_char(x + i as u32 *4 , y, caps_abc[29]),
                ')' => self.render_char(x + i as u32 *4 , y, caps_abc[30]),
                ':' => self.render_char(x + i as u32 *4 , y, caps_abc[31]),
                'a' => self.render_char(x + i as u32 *4 , y, abc[0]),
                'b' => self.render_char(x + i as u32 *4 , y, abc[1]),
                'c' => self.render_char(x + i as u32 *4 , y, abc[2]),
                'd' => self.render_char(x + i as u32 *4 , y, abc[3]),
                'e' => self.render_char(x + i as u32 *4 , y, abc[4]),
                'f' => self.render_char(x + i as u32 *4 , y, abc[5]),
                'g' => self.render_char(x + i as u32 *4 , y, abc[6]),
                'h' => self.render_char(x + i as u32 *4 , y, abc[7]),
                'i' => self.render_char(x + i as u32 *4 , y, abc[8]),
                'j' => self.render_char(x + i as u32 *4 , y, abc[9]),
                'k' => self.render_char(x + i as u32 *4 , y, abc[10]),
                'l' => self.render_char(x + i as u32 *4 , y, abc[11]),
                'm' => self.render_char(x + i as u32 *4 , y, abc[12]),
                'n' => self.render_char(x + i as u32 *4 , y, abc[13]),
                'o' => self.render_char(x + i as u32 *4 , y, abc[14]),
                'p' => self.render_char(x + i as u32 *4 , y, abc[15]),
                'q' => self.render_char(x + i as u32 *4 , y, abc[16]),
                'r' => self.render_char(x + i as u32 *4 , y, abc[17]),
                's' => self.render_char(x + i as u32 *4 , y, abc[18]),
                't' => self.render_char(x + i as u32 *4 , y, abc[19]),
                'u' => self.render_char(x + i as u32 *4 , y, abc[20]),
                'v' => self.render_char(x + i as u32 *4 , y, abc[21]),
                'w' => self.render_char(x + i as u32 *4 , y, abc[22]),
                'x' => self.render_char(x + i as u32 *4 , y, abc[23]),
                'y' => self.render_char(x + i as u32 *4 , y, abc[24]),
                'z' => self.render_char(x + i as u32 *4 , y, abc[25]),
                '0' => self.render_char(x + i as u32 *4 , y, numbers[0]),
                '1' => self.render_char(x + i as u32 *4 , y, numbers[1]),
                '2' => self.render_char(x + i as u32 *4 , y, numbers[2]),
                '3' => self.render_char(x + i as u32 *4 , y, numbers[3]),
                '4' => self.render_char(x + i as u32 *4 , y, numbers[4]),
                '5' => self.render_char(x + i as u32 *4 , y, numbers[5]),
                '6' => self.render_char(x + i as u32 *4 , y, numbers[6]),
                '7' => self.render_char(x + i as u32 *4 , y, numbers[7]),
                '8' => self.render_char(x + i as u32 *4 , y, numbers[8]),
                '9' => self.render_char(x + i as u32 *4 , y, numbers[9]),
                _ => continue,
            }
        }

        // fill in the remaining space with... space
        if text.len() < 38 {
            for i in 0..(38-text.len()) {
                self.render_char(x + i as u32 *4 + text.len() as u32 * 4 , y, caps_abc[27]);
            }
        }

    } // fn render_text


    pub fn render_char(&mut self, x : u32, y : u32, ch : u16) {
        //self.print_color = rand::thread_rng().gen::<u32>();
        for i in 0..15 {
            let bit = (ch & (2 << i)) >> i+1;

            if (y * 256 + x) < (256*360) {
                //println!("x: {}, y: {}", x, y);

                if bit == 1 && x < 256 {
                    self.vram[((self.char_to_y(i)+y as u32) * 256 + self.char_to_x(i)
                    + x as u32) as usize]
                    = self.print_color;
                }

                if bit == 0 && x < 256 {
                    self.vram[((self.char_to_y(i)+y as u32) * 256 + self.char_to_x(i)
                    + x as u32) as usize]
                    = self.textbg_color;
                }

            }
        }
    } // fn render_char

    pub fn char_to_x(&self, c : u8) -> u32 {
        (c - ((c/3)*3)) as u32
    }


    pub fn char_to_y(&self, c : u8) -> u32 {
        (c / 3) as u32
    }

    pub fn render_registers(&mut self, registers : &LinkedList<Vec<String>>) {
        let mut index = 0;

        for r in registers {

            if index == self.active_state {
                self.render_text(172, 5, &String::from("A"));
                self.render_text(184, 5, &r[0]);

                self.render_text(224, 5, &String::from("B"));
                self.render_text(236, 5, &r[1]);

                self.render_text(172, 25, &String::from("C"));
                self.render_text(184, 25, &r[2]);

                self.render_text(224, 25, &String::from("D"));
                self.render_text(236, 25, &r[3]);

                self.render_text(172, 44, &String::from("E"));
                self.render_text(184, 45, &r[4]);

                self.render_text(224, 45, &String::from("F"));
                self.render_text(236, 45, &r[5]);

                self.render_text(172, 65, &String::from("H"));
                self.render_text(184, 65, &r[6]);

                self.render_text(224, 65, &String::from("L"));
                self.render_text(236, 65, &r[7]);

                self.render_text(172, 85, &String::from("SP"));
                self.render_text(184, 85, &r[8]);

                //self.render_text(172, 105, &String::from("-------------------------------"));

                self.render_text(172, 125, &String::from("PC"));
                self.render_text(224, 125, &r[9]);

                self.render_text(172, 145, &String::from("FLAG"));
                self.render_text(224, 145, &r[10]);
            }
            index += 1;
        }


    } // fn render_registers

} // impl Debugger
