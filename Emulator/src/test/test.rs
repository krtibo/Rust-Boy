/*

                    ===    THIS IS RUST BOY    ===

    This is the test report generator and test mini framework for the
    Rust Boy Emulator.

    It takes the test cases from a txt file, processes them, and checks
    the given result.

    INPUT : ../../tests/tests.txt file
    OUTPUT : Generated index.html file which takes use of the css
             stylesheet.

    REQUIRED TEST FILE FORMAT :
        * instruction bytecode with the operand bytes (1 or 2 bytes),
          e.g. 3E 12  --> this means LD A, 12
        * result check part
            - to check a register, type in its name and its hoped result
              e.g. 3E 12 A 12  --> this does the instruction and checks
              if A is whether 0x12 or not
            - to check the flag, type in FLAG and its value in binary
              e.g. 2E 33 FLAG 10000000
            - to check a byte in RAM, type in RAM, its location in RAM
              and its value
              e.g. 3E 12 RAM 12 32

*/

use cpu::CPU;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Test {
    rb : CPU,
}

impl Test {
    pub fn new(cpu : CPU) -> Test {
        Test {
            rb : cpu,
        }
    }


fn test_writer(&self, result : bool, f : &mut File, data : (String, Vec<String>)) {
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

pub fn test_parser(& mut self){
    let mut out = File::create("./tests/index.html").expect("Can't create file!");

    out.write_all(b"<!DOCTYPE html>
        <html >
        <head>
        <meta charset=\"UTF-8\">
        <title>Rust Boy Test Report</title>

            <link href=\"https://afeld.github.io/emoji-css/emoji.css\" rel=\"stylesheet\">
            <link rel=\"stylesheet\" href=\"css/style.css\">


        </head>

        <body>
        <h1>Rust Boy <span class=\"yellow\">Test Report</pan></h1>
        <h2>Created with Rust Boy Emulator by <a>Tibor Krizsak</a></h2>

        <br></br>

        <h2><a><span class=\"ldblue\">LD</pan></a> instruction tests</h2>

        <table class=\"container\">
            <thead>
                <tr>
                    <th><h1>Result</h1></th>
                    <th><h1>Instruction</h1></th>
                    <th><h1>Flags</h1></th>
                    <th><h1>A</h1></th>
                    <th><h1>B</h1></th>
                    <th><h1>C</h1></th>
                    <th><h1>D</h1></th>
                    <th><h1>E</h1></th>
                    <th><h1>F</h1></th>
                    <th><h1>H</h1></th>
                    <th><h1>L</h1></th>
                    <th><h1>SP</h1></th>
                    <th><h1>PC</h1></th>
                    <th><h1>SP size</h1></th>
                </tr>
            </thead>
	<tbody>").unwrap();




    let f = File::open("./tests/tests.txt")
        .expect("Error with test loading!");
    let tests = BufReader::new(&f);

    for line in tests.lines() {
        let line_string = line.unwrap();
        let iter = line_string.split_whitespace();
        let vec_str : Vec<&str> = iter.collect();
        let mut bytes : [u8; 3] = [0; 3];
        //println!("{:?}", vec_str);

        if line_string.contains("RAM"){

            if vec_str.len() == 4 {
                bytes = [u8::from_str_radix(vec_str[0], 16).unwrap(), 0, 0];
            }

            if vec_str.len() == 5 {
                bytes = [
                u8::from_str_radix(vec_str[0], 16).unwrap(),
                u8::from_str_radix(vec_str[1], 16).unwrap(), 0];
            }

            if vec_str.len() == 6 {
                bytes = [
                    u8::from_str_radix(vec_str[0], 16).unwrap(),
                    u8::from_str_radix(vec_str[1], 16).unwrap(),
                    u8::from_str_radix(vec_str[2], 16).unwrap()];
            }

            let data = self.rb.test_bytes(&bytes);

            let b1 : u8 = vec_str[vec_str.len()-2].parse().unwrap();
            let b2 : u8 = vec_str[vec_str.len()-1].parse().unwrap();
            self.test_writer(
                    if self.rb.RAM[b1 as usize] == b2
                    {true} else {false},
                    &mut out,
                    data);

            continue;

        } else {

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
        }



        let data = self.rb.test_bytes(&bytes);

        match vec_str[vec_str.len()-2] {
            "A" => self.test_writer(
                    if self.rb.A == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "B" => self.test_writer(
                    if self.rb.B == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "C" => self.test_writer(
                    if self.rb.C == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "D" => self.test_writer(
                    if self.rb.D == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "E" => self.test_writer(
                    if self.rb.E == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "F" => self.test_writer(
                    if self.rb.F == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false}, 
                    &mut out,
                    data),
            "H" => self.test_writer(
                    if self.rb.H == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "L" => self.test_writer(
                    if self.rb.L == u8::from_str_radix(vec_str[vec_str.len()-1], 16).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            "FLAG" => self.test_writer(
                    if self.rb.F == u8::from_str_radix(vec_str[vec_str.len()-1], 2).unwrap()
                    {true} else {false},
                    &mut out,
                    data),
            _ => println!("NAO"),
        }
    }

    out.write_all(b"</tbody></table></body></html>").unwrap();
    }
}
