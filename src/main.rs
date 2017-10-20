pub mod cpu;
use cpu::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
    

    
    // let mut rust_boy : CPU = CPU::new();
    // rust_boy.load_rom(String::from("./rom/test.gb"));

    /*
    rust_boy.load_rom(String::from("/home/tiibo/Cloud/PROGRAMMING/Rust_Boy/rom/boot_rom.gb"));
    rust_boy.cycle();
    */


    test_parser();


    /* 
    
    -- 8-BIT HALF CARRY DETECTING EXAMPLE --

    let a : u8 = 0b00111100;
    let b : u8 = 0b00010000;

    if (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 {
        println!("HALF CARRY DUDE");
    } 
    else {
        println!("NO HALF CARRY");
    }


    -- 8-BIG HALB BORROW DETECTING EXAMPLE --

    self.reg.flag(H, (a & 0x0F) < (b & 0x0F) + c);

    let a : u8 = 0b00111100;
    let b : u8 = 0b00010000;

    if (a & 0x0F) < (b & 0x0F) {
        println!("HALF BORROW DUDE");
    } 

    SBC EXAMPLE

    if (cpu.FLAG & 0b0001_0000) >> 4 == 1 {     // C FLAG ?= 1
        if (a & 0x0F) < (b & 0x0F) + 1 {
            println!("HALF BORROW DUDE");
        } 
    }





    
    -- 16-BIT HALF CARRY DETECTING EXAMPLE --

    let a : u16 = 0b0000_1000_0000_0000;
    println!("{:b}", a);
    let b : u16 = 0b0000_1000_0000_0000;
    println!("{:b}", b);

    if (((a & 0x0fff) + (b & 0x0fff)) & 0x1000) == 0x1000 {
        println!("HALF CARRY DUDE");
    } 
    else {
        println!("NO HALF CARRY");
    }

    */

}

fn test_writer(result : bool, f : &mut File, data : (String, Vec<String>)) {
    f.write_all(b"<tr>").unwrap();

    if result {
        f.write_all(b"<td> <i class=\"em em-white_check_mark\"></i> </td>").unwrap();
    } else {
        f.write_all(b"<td> <i class=\"em em-x\"></i> </td>").unwrap();
    }

    f.write_all(format!("<td> {} </td>", data.0).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[10]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[0]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[1]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[2]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[3]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[4]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[5]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[6]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[7]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[8]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[9]).as_bytes()).unwrap();
    f.write_all(format!("<td> {} </td>", data.1[11]).as_bytes()).unwrap();


    f.write_all(b"</tr>").unwrap();
}

fn test_parser(){
    let mut rb : CPU = CPU::new();
    let mut out = File::create("foo.html").expect("Can't create file!");
    out.write_all(b"<h1> Rust Boy test document </h1>").unwrap();
    out.write_all(
          b"
            <head>
            <link href=\"https://afeld.github.io/emoji-css/emoji.css\" rel=\"stylesheet\">
            <meta charset='utf-8'>
                <style>
                    table, th, td {
                    border: 1px solid black;
                    border-collapse: collapse;
                    }
                    th, td {
                    padding: 15px;
                    }
                    th {
                        text-align: left;
                    }
                </style>
             </head>
          
            <table style=\"width:100%\">
                <tr>
                    <th>Result</th>
                    <th>Instruction</th>
                    <th>Flags</th> 
                    <th>A</th>
                    <th>B</th>
                    <th>C</th>
                    <th>D</th>
                    <th>E</th>
                    <th>F</th>
                    <th>H</th>
                    <th>L</th>
                    <th>SP</th>
                    <th>PC</th>
                    <th>SP size</th>
                </tr>").unwrap();

    let f = File::open("./tests.txt")
        .expect("Error with test loading!");
    let tests = BufReader::new(&f);
    
    for line in tests.lines() {
        let line_string = line.unwrap();
        let iter = line_string.split_whitespace();
        let vec_str : Vec<&str> = iter.collect();
        let mut bytes : [u8; 3] = [0; 3];
        //println!("{:?}", vec_str);

        if vec_str.len() == 3 {
            bytes = [u8::from_str_radix(vec_str[0], 16).unwrap(), 0, 0];
        }

        if vec_str.len() == 4 {
            bytes = [
            u8::from_str_radix(vec_str[0], 16).unwrap(),
            u8::from_str_radix(vec_str[1], 16).unwrap(), 0];
        }

        if vec_str.len() == 5 {
            bytes = [
                u8::from_str_radix(vec_str[0], 16).unwrap(),
                u8::from_str_radix(vec_str[1], 16).unwrap(),
                u8::from_str_radix(vec_str[2], 16).unwrap()];
        }

        let data = rb.test_bytes(&bytes);


        match vec_str[vec_str.len()-2] {
            "A" => test_writer(
                    if rb.A == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "B" => test_writer(
                    if rb.B == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "C" => test_writer(
                    if rb.C == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "D" => test_writer(
                    if rb.D == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "E" => test_writer(
                    if rb.E == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "F" => test_writer(
                    if rb.F == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "H" => test_writer(
                    if rb.H == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            "L" => test_writer(
                    if rb.L == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap() 
                    {true} else {false}, 
                    &mut out, 
                    data),
            _ => println!("NAO"),
        }
    }

    out.write_all(b"</table>").unwrap();
}
