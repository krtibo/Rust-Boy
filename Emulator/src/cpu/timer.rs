/*************************************************************************

                    ===    THIS IS RUST BOY    ===

    This is the Timer class. It synchronizes every timer in the Game Boy.
    If the timer hits the u8 limit, it makes an IRQ. The update function
    should be called in every CPU cycle.

    PARAMETERS :
        * CPU
        * Current cycle

*************************************************************************/

#![allow(non_snake_case)]

use cpu::CPU;
use interrupt::Interrupt;

pub struct Timer {
    timer_counter : u32,
    divider_register : u16,
    pub TMC : u16,
    TMA : u16,
    TIMA : u16,
    interrupt : Interrupt,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            timer_counter : 0,
            divider_register : 0,
            TMC : 0xFF07,
            TMA : 0xFF06,
            TIMA : 0xFF05,
            interrupt : Interrupt::new(),
        }
    }

    pub fn update(&mut self, cpu : &mut CPU, cycle : u8) {
        let interrupt : Interrupt = Interrupt::new();
        self.div_reg(cpu, cycle);

        // Is the clock enabled?
        if CPU::get_bit(2, cpu.RAM[self.TMC as usize]) {
            // Is enough time passed to update the timer?
            if cycle as u32 > self.timer_counter  {
                // Update the current frequency.
                self.update_freq(cpu);
                // Check for the overflow.
                if cpu.RAM[self.TIMA as usize] >= 255 {
                    // If an overflow occurs, load TMA to TIMA,
                    let TMA = cpu.RAM[self.TMA as usize];
                    cpu.write_ram(self.TIMA, TMA);
                    // and send an interrupt request.
                    self.interrupt.IRQ(cpu, 2);
                } else {
                    // If no overflow, increment the TIMA.
                    let TIMA = cpu.RAM[self.TIMA as usize];
                    cpu.write_ram(self.TIMA, TIMA + 1);
                }
            } else {
                // Sync the time with the CPU.
                self.timer_counter -= cycle as u32;
            }
        }
    }

    pub fn update_freq(&mut self, cpu : &mut CPU) {
        let new_freq = cpu.RAM[self.TMC as usize] & 0x3;

        match new_freq {
            0 => self.timer_counter = 1024,
            1 => self.timer_counter = 16,
            2 => self.timer_counter = 64,
            3 => self.timer_counter = 256,
            _ => return
        }
    }

    fn div_reg(&mut self, cpu : &mut CPU, cycle : u8) {
        self.divider_register += cycle as u16;
        if self.divider_register >= 255 && cpu.RAM[0xFF04] < 255 {
            self.divider_register = 0;
            cpu.RAM[0xFF04] += 1;
        }

        if self.divider_register >= 255 && cpu.RAM[0xFF04] == 255 {
            self.divider_register = 0;
            cpu.RAM[0xFF04] = 0;
        }
    }

}
