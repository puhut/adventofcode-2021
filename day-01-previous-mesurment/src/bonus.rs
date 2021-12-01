use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let mut previous_first_number = String::new();
    let mut previous_second_number = String::new();

    if let Ok(lines) = read_lines(filename){
        println!("Starting!:::");

        let path = "result.txt";
        let mut output = File::create(path).unwrap();
        
        for line in lines {
            if let Ok(number) = line {
                if !previous_first_number.trim().is_empty() && !previous_second_number.trim().is_empty() {
                    let p1_n = i32::from_str(&previous_first_number).unwrap();
                    let p2_n = i32::from_str(&previous_second_number).unwrap();
                    let o_n = i32::from_str(&number).unwrap();
                    let total_3 = p1_n + p2_n + o_n;
                    output.write_all(format!("{}\n", total_3).as_bytes()).unwrap();
                }

                println!("test: {} {} {}", previous_first_number, previous_second_number, number);

                previous_first_number = previous_second_number;
                previous_second_number = number;

            }
        }
    }
}