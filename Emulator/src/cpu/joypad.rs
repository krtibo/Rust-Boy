/*************************************************************************

                    ===    THIS IS RUST BOY    ===

    This is the joypad handler class. It scans for any pressed and un-
    pressed buttons, and if it's in a different state than it was before,
    an IRQ is requested. The scan_window_button_pressed function should be
    called in every CPU cycles.

    PARAMETERS :
        * A Window, for scanning keystrokes
        * CPU (for RAM writing)

***************************************************************************/

#![allow(unused_assignments)]
#![allow(non_snake_case)]

use cpu::CPU;
use interrupt::Interrupt;
extern crate minifb;
use self::minifb::{Window, Key};

pub struct Joypad {
    joypad_state : u8,
    interrupt : Interrupt,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            joypad_state : 0xFF,
            interrupt : Interrupt::new(),
        }
    }

    pub fn scan_window_button_pressed(&mut self, window : &Window, cpu : &mut CPU) {

        if window.is_key_down(Key::D) {
            self.pressed_button(0, cpu);
        } else { self.released_button(0); }

        if window.is_key_down(Key::A) {
            self.pressed_button(1, cpu);
        } else { self.released_button(1); }

        if window.is_key_down(Key::W) {
            self.pressed_button(2, cpu);
        } else { self.released_button(2); }

        if window.is_key_down(Key::S) {
            self.pressed_button(3, cpu);
        } else { self.released_button(3); }

        if window.is_key_down(Key::J) {
            self.pressed_button(4, cpu);
        } else { self.released_button(4); }

        if window.is_key_down(Key::K) {
            self.pressed_button(5, cpu);
        } else { self.released_button(5); }

        if window.is_key_down(Key::Space) {
            self.pressed_button(6, cpu);
        } else { self.released_button(6); }

        if window.is_key_down(Key::RightShift) {
            self.pressed_button(7, cpu);
        } else { self.released_button(7); }

    } // fn scan_window_button_pressed


    pub fn pressed_button(&mut self, button : u8, cpu : &mut CPU) {

        let mut changed_state = false;
        let mut button_type = true;
        let mut IRQ = false;
        let interrupt : Interrupt = Interrupt::new();

        if CPU::get_bit(button, self.joypad_state) == false {
            changed_state = true;
        }

        self.joypad_state = CPU::reset_bit(button, self.joypad_state);

        if button > 3 {
            button_type = true;
        } else {
            button_type = false;
        }

        let current_joypad_state : u8 = cpu.RAM[0xFF00];

        if button_type && !CPU::get_bit(5, current_joypad_state) {
            IRQ = true;
        }

        if !button_type && !CPU::get_bit(4, current_joypad_state) {
            IRQ = true;
        }

        if !changed_state && IRQ {
            self.interrupt.IRQ(cpu, 4);
        }
    } // fn pressed_button

    pub fn released_button(&mut self, button : u8) {
        self.joypad_state = CPU::set_bit(button, self.joypad_state);
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
    } // fn update_state

} // impl Joypad
