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
        self.opc[0x60] = Opcode::ld_r1r2_60;
        self.opc[0x61] = Opcode::ld_r1r2_61;
        self.opc[0x62] = Opcode::ld_r1r2_62;
        self.opc[0x63] = Opcode::ld_r1r2_63;
        self.opc[0x64] = Opcode::ld_r1r2_64;
        self.opc[0x65] = Opcode::ld_r1r2_65;
        self.opc[0x66] = Opcode::ld_r1r2_66;
        self.opc[0x68] = Opcode::ld_r1r2_68;
        self.opc[0x69] = Opcode::ld_r1r2_69;
        self.opc[0x6a] = Opcode::ld_r1r2_6a;
        self.opc[0x6b] = Opcode::ld_r1r2_6b;
        self.opc[0x6c] = Opcode::ld_r1r2_6c;
        self.opc[0x6d] = Opcode::ld_r1r2_6d;
        self.opc[0x6e] = Opcode::ld_r1r2_6e;
        // LD A,n
        self.opc[0x0a] = Opcode::ld_an_0a;
        self.opc[0x1a] = Opcode::ld_an_1a;
        self.opc[0xfa] = Opcode::ld_an_fa;
        self.opc[0x3e] = Opcode::ld_an_3e;
        // LD n,A
        self.opc[0x47] = Opcode::ld_na_47;
        self.opc[0x4f] = Opcode::ld_na_4f;
        self.opc[0x57] = Opcode::ld_na_57;
        self.opc[0x5f] = Opcode::ld_na_5f;
        self.opc[0x67] = Opcode::ld_na_67;
        self.opc[0x6f] = Opcode::ld_na_6f;
        self.opc[0x02] = Opcode::ld_na_02;
        self.opc[0x12] = Opcode::ld_na_12;
        self.opc[0x77] = Opcode::ld_na_77;
        self.opc[0xea] = Opcode::ld_na_ea;

        self.opc[0xf2] = Opcode::ld_ac_f2;
        self.opc[0xe2] = Opcode::ld_ca_e2;
        self.opc[0x3a] = Opcode::ldd_ahl_3a;
        self.opc[0x32] = Opcode::ldd_hla_32;
        self.opc[0x2a] = Opcode::ldi_ahl_2a;
        self.opc[0x22] = Opcode::ldi_hla_22;
        self.opc[0xe0] = Opcode::ldh_na_e0;
        self.opc[0xf0] = Opcode::ldh_an_f0;
        // 16-bit loads
        self.opc[0x01] = Opcode::ld_nnn_01;
        self.opc[0x11] = Opcode::ld_nnn_11;
        self.opc[0x21] = Opcode::ld_nnn_21;
        self.opc[0x31] = Opcode::ld_nnn_31;
        self.opc[0xf9] = Opcode::ld_sphl_f9;
        self.opc[0x08] = Opcode::ld_nnsp_08;


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


    fn ld_r1r2_60(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.B;

        self.last_instruction = "LD H, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_61(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.C;

        self.last_instruction = "LD H, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_62(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.D;

        self.last_instruction = "LD H, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_63(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.E;

        self.last_instruction = "LD H, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_64(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.H;

        self.last_instruction = "LD H, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_65(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.L;

        self.last_instruction = "LD H, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_66(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD H, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_68(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.B;

        self.last_instruction = "LD L, B";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_69(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.C;

        self.last_instruction = "LD L, C";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.D;

        self.last_instruction = "LD L, D";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6b(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.E;

        self.last_instruction = "LD L, E";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6c(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.H;

        self.last_instruction = "LD L, H";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6d(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.L;

        self.last_instruction = "LD L, L";
        self.operand_mode = 0;
        4
    }


    fn ld_r1r2_6e(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.RAM[
                Opcode::byte_cat(cpu.H, cpu.L)
                as usize];

        self.last_instruction = "LD L, (HL)";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_70(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.B;

        self.last_instruction = "LD (HL), B";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_71(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.C;

        self.last_instruction = "LD (HL), C";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_72(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.D;

        self.last_instruction = "LD (HL), D";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_73(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.E;

        self.last_instruction = "LD (HL), E";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_74(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.H;

        self.last_instruction = "LD (HL), H";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_75(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.L;

        self.last_instruction = "LD (HL), L";
        self.operand_mode = 0;
        8
    }


    fn ld_r1r2_36(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = data;

        self.rhs = data as u16;
        self.last_instruction = "LD (HL),";
        self.operand_mode = 1;
        12
    }


    fn ld_an_0a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[
                Opcode::byte_cat(cpu.B, cpu.C)
                as usize];

        self.last_instruction = "LD A, (BC)";
        self.operand_mode = 0;
        8
    }


    fn ld_an_1a(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[
                Opcode::byte_cat(cpu.D, cpu.E)
                as usize];

        self.last_instruction = "LD A, (DE)";
        self.operand_mode = 0;
        8
    }


    fn ld_an_fa(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);

        cpu.A = cpu.RAM[
                Opcode::byte_cat(data_h, data_l)
                as usize];

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD A,";
        self.operand_mode = 2;
        16
    }


    fn ld_an_3e(&mut self, cpu : &mut CPU) -> u8 {
        let data : u8 = self.fetch(cpu);
        cpu.A = data;

        self.rhs = data as u16;
        self.last_instruction = "LD A,";
        self.operand_mode = 1;
        8
    }


    fn ld_na_47(&mut self, cpu : &mut CPU) -> u8 {
        cpu.B = cpu.A;

        self.last_instruction = "LD B,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_4f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.C = cpu.A;

        self.last_instruction = "LD C,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_57(&mut self, cpu : &mut CPU) -> u8 {
        cpu.D = cpu.A;

        self.last_instruction = "LD D,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_5f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.E = cpu.A;

        self.last_instruction = "LD E,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_67(&mut self, cpu : &mut CPU) -> u8 {
        cpu.H = cpu.A;

        self.last_instruction = "LD H,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_6f(&mut self, cpu : &mut CPU) -> u8 {
        cpu.L = cpu.A;

        self.last_instruction = "LD L,A";
        self.operand_mode = 0;
        4
    }


    fn ld_na_02(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.B, cpu.C) as usize] = cpu.A;

        self.last_instruction = "LD (BC),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_12(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.D, cpu.E) as usize] = cpu.A;

        self.last_instruction = "LD (DE),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_77(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[Opcode::byte_cat(cpu.H, cpu.L) as usize] = cpu.A;

        self.last_instruction = "LD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ld_na_ea(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);

        cpu.RAM[Opcode::byte_cat(data_h, data_l) as usize] = cpu.A;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD (nn),A";
        self.operand_mode = 2;
        16
    }


    fn ld_ac_f2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.A = cpu.RAM[0xFF00 + cpu.C as usize];

        self.last_instruction = "LD A,(C)";
        self.operand_mode = 0;
        8
    }


    fn ld_ca_e2(&mut self, cpu : &mut CPU) -> u8 {
        cpu.RAM[0xFF00 + cpu.C as usize] = cpu.A;

        self.last_instruction = "LD (C),A";
        self.operand_mode = 0;
        8
    }


    fn ldd_ahl_3a(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.A = cpu.RAM[hl as usize];
        hl -= 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn ldd_hla_32(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.RAM[hl as usize] = cpu.A;
        hl -= 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ldi_ahl_2a(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.A = cpu.RAM[hl as usize];
        hl += 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDI A,(HL)";
        self.operand_mode = 0;
        8
    }


    fn ldi_hla_22(&mut self, cpu : &mut CPU) -> u8 {
        let mut hl : u16 = Opcode::byte_cat(cpu.H, cpu.L);
        cpu.RAM[hl as usize] = cpu.A;
        hl += 1;
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.last_instruction = "LDD (HL),A";
        self.operand_mode = 0;
        8
    }


    fn ldh_na_e0(&mut self, cpu : &mut CPU) -> u8 {
        let n : u8 = self.fetch(cpu);
        cpu.RAM[0xFF00 + n as usize] = cpu.A;

        self.rhs = n as u16;
        self.last_instruction = "LDH (n),A";
        self.operand_mode = 1;
        12
    }


    fn ldh_an_f0(&mut self, cpu : &mut CPU) -> u8 {
        let n : u8 = self.fetch(cpu);
        cpu.A = cpu.RAM[0xFF00 + n as usize];

        self.rhs = n as u16;
        self.last_instruction = "LDH A, (n)";
        self.operand_mode = 1;
        12
    }


    fn ld_nnn_01(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let bc : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.B = (bc >> 8) as u8;
        cpu.C = (bc & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD BC,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_nnn_11(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let de : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.D = (de >> 8) as u8;
        cpu.E = (de & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD DE,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_nnn_21(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let hl : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.H = (hl >> 8) as u8;
        cpu.L = (hl & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD HL,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_nnn_31(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        let sp : u16 = Opcode::byte_cat(data_h, data_l);
        cpu.SP = sp;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD SP,nn";
        self.operand_mode = 2;
        12
    }


    fn ld_sphl_f9(&mut self, cpu : &mut CPU) -> u8 {
        cpu.SP = Opcode::byte_cat(cpu.H, cpu.L);

        self.last_instruction = "LD SP,HL";
        self.operand_mode = 0;
        8
    }


    fn ld_nnsp_08(&mut self, cpu : &mut CPU) -> u8 {
        let data_l : u8 = self.fetch(cpu);
        let data_h : u8 = self.fetch(cpu);
        cpu.RAM[Opcode::byte_cat(data_h, data_l) as usize] =
        (cpu.SP >> 8) as u8;

        cpu.RAM[(Opcode::byte_cat(data_h, data_l) + (1 as u16)) as usize] =
        (cpu.SP & 0x00FF) as u8;

        self.lhs = data_h as u16;
        self.rhs = data_l as u16;
        self.last_instruction = "LD (nn),SP";
        self.operand_mode = 2;
        20
    }

}
