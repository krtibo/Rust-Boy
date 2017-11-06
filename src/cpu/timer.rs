use cpu::CPU;

pub struct Timer {
    timer_counter : u8,
    divider_register : u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            timer_counter : 0,
            divider_register : 0,
        }
    }

    pub fn update(&mut self, cpu : &mut CPU) {
        
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