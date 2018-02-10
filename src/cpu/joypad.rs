#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;
extern crate minifb;

use self::minifb::{Window, Key, KeyRepeat};

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

        if window.is_key_pressed(Key::D, KeyRepeat::Yes) {
            self.pressed_button(0, cpu);
            println!("D pressed");
        } else { self.released_button(0); }

        if window.is_key_pressed(Key::A, KeyRepeat::Yes) {
            self.pressed_button(1, cpu);
            println!("A pressed");
        } else { self.released_button(1); }

        if window.is_key_pressed(Key::W, KeyRepeat::Yes) {
            self.pressed_button(2, cpu);
            println!("W pressed");
        } else { self.released_button(2); }

        if window.is_key_pressed(Key::S, KeyRepeat::Yes) {
            self.pressed_button(3, cpu);
            println!("S pressed");
        } else { self.released_button(3); }

        if window.is_key_pressed(Key::J, KeyRepeat::Yes) {
            self.pressed_button(4, cpu);
            println!("J pressed");
        } else { self.released_button(4); }

        if window.is_key_pressed(Key::K, KeyRepeat::Yes) {
            self.pressed_button(5, cpu);
            println!("K pressed");
        } else { self.released_button(5); }

        if window.is_key_pressed(Key::Space, KeyRepeat::Yes) {
            self.pressed_button(6, cpu);
            println!("Space pressed");
        } else { self.released_button(6); }

        if window.is_key_pressed(Key::RightShift, KeyRepeat::Yes) {
            self.pressed_button(7, cpu);
            println!("Right Shift pressed");
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
        current_joypad_state ^= 0xFF;

        if !Joypad::get_bit(4, current_joypad_state) {

            let mut upper_joypad = self.joypad_state >> 4;
            upper_joypad |= 0xF0;
            current_joypad_state &= upper_joypad;

        } else if !Joypad::get_bit(5, current_joypad_state) {

            let mut lower_joypad = self.joypad_state & 0xF;
            lower_joypad |= 0xF0;
            current_joypad_state &= lower_joypad;
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
