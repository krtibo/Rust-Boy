/*
Button    76543210   val
------------------------
Start     11100111 = 231
Select    11101011 = 235
B         11101101 = 237
A         11101110 = 238

Down      11010111 = 215
Up        11011011 = 219
Left      11011101 = 221
Right     11011110 = 222
*/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;
extern crate minifb;

use self::minifb::{Window, Key};

pub struct Joypad {
    joypad_state : u8,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            joypad_state : 0xFF,
        }
    }

    pub fn scan_window_button_pressed(&mut self, window : &Window, cpu : &mut CPU) {

        if window.is_key_down(Key::D) {
            self.pressed_button(0, cpu);
            //println!("D pressed");
        } else { self.released_button(0); }

        if window.is_key_down(Key::A) {
            self.pressed_button(1, cpu);
            //println!("A pressed");
        } else { self.released_button(1); }

        if window.is_key_down(Key::W) {
            self.pressed_button(2, cpu);
            //println!("W pressed");
        } else { self.released_button(2); }

        if window.is_key_down(Key::S) {
            self.pressed_button(3, cpu);
            //println!("S pressed");
        } else { self.released_button(3); }

        if window.is_key_down(Key::J) {
            self.pressed_button(4, cpu);
            //println!("J pressed");
        } else { self.released_button(4); }

        if window.is_key_down(Key::K) {
            self.pressed_button(5, cpu);
            //println!("K pressed");
        } else { self.released_button(5); }

        if window.is_key_down(Key::Space) {
            self.pressed_button(6, cpu);
            //println!("Space pressed");
        } else { self.released_button(6); }

        if window.is_key_down(Key::RightShift) {
            self.pressed_button(7, cpu);
            //println!("Right Shift pressed");
        } else { self.released_button(7); }

    }


    pub fn pressed_button(&mut self, button : u8, cpu : &mut CPU) {

        //println!("{:X} pressed", cpu.RAM[0xFF00]);

        let mut changed_state = false;
        let mut button_type = true;
        let mut IRQ = false;

        if Joypad::get_bit(button, self.joypad_state) == false {
            changed_state = true;
        }

        self.joypad_state = Joypad::reset_bit(button, self.joypad_state);

        if button > 3 {
            button_type = true;
        } else {
            button_type = false;
        }

        let current_joypad_state : u8 = cpu.RAM[0xFF00];

        if button_type && !Joypad::get_bit(5, current_joypad_state) {
            IRQ = true;
        }

        if !button_type && !Joypad::get_bit(4, current_joypad_state) {
            IRQ = true;
        }

        if !changed_state && IRQ {
            cpu.IRQ(4);
        }
    }

    pub fn released_button(&mut self, button : u8) {
        self.joypad_state = Joypad::set_bit(button, self.joypad_state);
    }

    pub fn update_state(&mut self, cpu : &mut CPU) -> u8 {
        let mut current_joypad_state : u8 = cpu.RAM[0xFF00];

        if current_joypad_state == 0x20 {
            current_joypad_state = !current_joypad_state;
            current_joypad_state &= self.joypad_state & 0x0F;
        }

        if current_joypad_state == 0x10 {
            current_joypad_state = !current_joypad_state;
            current_joypad_state &= self.joypad_state >> 4;
        }

        current_joypad_state
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

    fn get_bit(n : u8, reg : u8) -> bool {
        let mask : u8 = 1 << n;

        if reg & mask == 0 {
            false
        } else {
            true
        }
    }

}
