#![allow(dead_code)]
#![allow(unused_variables)]

use cpu::CPU;

pub struct Opcode {
    pub lhs : u16,
    pub rhs : u16,
    pub operand_mode : u8,
    pub opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub cb_opc : [fn(&mut Opcode, &mut CPU) -> u8; 256],
    pub last_instruction :  &'static str,

}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            lhs : 0,
            rhs : 0,
            operand_mode : 0,
            opc : [Opcode::default; 256],
            cb_opc : [Opcode::default; 256],
            last_instruction : "",

        }
    }


    pub fn init(&mut self) {
        // LD nnn
        self.opc[0x06] = Opcode::ld_nnn_06;
        self.opc[0x0e] = Opcode::ld_nnn_0e;
        self.opc[0x16] = Opcode::ld_nnn_16;
        self.opc[0x1e] = Opcode::ld_nnn_1e;
        self.opc[0x26] = Opcode::ld_nnn_26;
        self.opc[0x2e] = Opcode::ld_nnn_2e;
        // LD r1,r2
        self.opc[0x7f] = Opcode::ld_r1r2_7f;
        self.opc[0x78] = Opcode::ld_r1r2_78;
        self.opc[0x79] = Opcode::ld_r1r2_79;
        self.opc[0x7a] = Opcode::ld_r1r2_7a;
        self.opc[0x7b] = Opcode::ld_r1r2_7b;
        self.opc[0x7c] = Opcode::ld_r1r2_7c;
        self.opc[0x7d] = Opcode::ld_r1r2_7d;
        self.opc[0x7e] = Opcode::ld_r1r2_7e;
        self.opc[0x40] = Opcode::ld_r1r2_40;
        self.opc[0x41] = Opcode::ld_r1r2_41;
        self.opc[0x42] = Opcode::ld_r1r2_42;
        self.opc[0x43] = Opcode::ld_r1r2_43;
        self.opc[0x44] = Opcode::ld_r1r2_44;
        self.opc[0x45] = Opcode::ld_r1r2_45;
        self.opc[0x46] = Opcode::ld_r1r2_46;
        self.opc[0x48] = Opcode::ld_r1r2_48;
        self.opc[0x49] = Opcode::ld_r1r2_49;
        self.opc[0x4a] = Opcode::ld_r1r2_4a;
        self.opc[0x4b] = Opcode::ld_r1r2_4b;
        self.opc[0x4c] = Opcode::ld_r1r2_4c;
        self.opc[0x4d] = Opcode::ld_r1r2_4d;
        self.opc[0x4e] = Opcode::ld_r1r2_4e;
        self.opc[0x50] = Opcode::ld_r1r2_50;
        self.opc[0x51] = Opcode::ld_r1r2_51;
        self.opc[0x52] = Opcode::ld_r1r2_52;
        self.opc[0x53] = Opcode::ld_r1r2_53;
        self.opc[0x54] = Opcode::ld_r1r2_54;
        self.opc[0x55] = Opcode::ld_r1r2_55;
        self.opc[0x56] = Opcode::ld_r1r2_56;
        self.opc[0x58] = Opcode::ld_r1r2_58;
        self.opc[0x59] = Opcode::ld_r1r2_59;
        self.opc[0x5a] = Opcode::ld_r1r2_5a;
        self.opc[0x5b] = Opcode::ld_r1r2_5b;
        self.opc[0x5c] = Opcode::ld_r1r2_5c;
        self.opc[0x5d] = Opcode::ld_r1r2_5d;
        self.opc[0x5e] = Opcode::ld_r1r2_5e;
    }


    // fetch the opcode and the byte after
    pub fn fetch(&mut self, cpu : &mut CPU) -> u8 {
        // increase the Program Counter
        let ret = cpu.RAM[(cpu.PC) as usize];
        cpu.PC += 1;
        ret
    }


    pub fn execute(&mut self, cpu : &mut CPU) -> u8 {
        self.opc[self.fetch(cpu) as usize](self, cpu)
    }

    fn byte_cat(h : u8, l : u8) -> u16 {
        let high : u16 = (h as u16) << 8;
        let low : u16 = l as u16;
        let hl : u16 = high | low;
        hl
    }

    //////  O P C O D E S  //////


    fn default(&mut self, cpu : &mut CPU) -> u8 {
        //println!("DEFAULT");
        self.last_instruction = "Default";
        self.operand_mode = 0;
        1
    }


    fn ld_nnn_06(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.B = data;
        self.rhs = data as u16;
        self.last_instruction = "LD B,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_0e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.C = data;
        self.rhs = data as u16;
        self.last_instruction = "LD C,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_16(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.D = data;
        self.rhs = data as u16;
        self.last_instruction = "LD D,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_1e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.E = data;
        self.rhs = data as u16;
        self.last_instruction = "LD E,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_26(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.H = data;
        self.rhs = data as u16;
        self.last_instruction = "LD H,";
        self.operand_mode = 1;
        8
    }


    fn ld_nnn_2e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.L = data;
        self.rhs = data as u16;
        self.last_instruction = "LD L,";
        self.operand_mode = 1;
        8
    }








    fn ld_r1r2_7f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.A;

        self.last_instruction = "LD A, A";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_78(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.B;

        self.last_instruction = "LD A, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_79(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.C;

        self.last_instruction = "LD A, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.D;

        self.last_instruction = "LD A, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.E;

        self.last_instruction = "LD A, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.H;

        self.last_instruction = "LD A, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.L;

        self.last_instruction = "LD A, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_7e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD A, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_40(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.B;

        self.last_instruction = "LD B, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_41(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.C;

        self.last_instruction = "LD B, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_42(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.D;

        self.last_instruction = "LD B, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_43(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.E;

        self.last_instruction = "LD B, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_44(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.H;

        self.last_instruction = "LD B, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_45(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.L;

        self.last_instruction = "LD B, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_46(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD B, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_48(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.B;

        self.last_instruction = "LD C, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_49(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.C;

        self.last_instruction = "LD C, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.D;

        self.last_instruction = "LD C, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.E;

        self.last_instruction = "LD C, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.H;

        self.last_instruction = "LD C, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.L;

        self.last_instruction = "LD C, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_4e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD C, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_50(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.B;

        self.last_instruction = "LD D, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_51(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.C;

        self.last_instruction = "LD D, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_52(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.D;

        self.last_instruction = "LD D, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_53(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.E;

        self.last_instruction = "LD D, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_54(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.H;

        self.last_instruction = "LD D, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_55(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.L;

        self.last_instruction = "LD D, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_56(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD D, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_58(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.B;

        self.last_instruction = "LD E, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_59(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.C;

        self.last_instruction = "LD E, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.D;

        self.last_instruction = "LD E, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.E;

        self.last_instruction = "LD E, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.H;

        self.last_instruction = "LD E, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.L;

        self.last_instruction = "LD E, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_5e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD E, (HL)";
        self.operand_mode = 0;
        8
    }


}
