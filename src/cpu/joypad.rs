#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;
extern crate minifb;

use self::minifb::{WindowOptions, Window, Scale};

pub struct Joypad {
    joypad_state : u8,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            joypad_state : 0xFF,
        }
    }

    pub fn pressed_button(&mut self, button : u8, cpu : &mut CPU) {

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