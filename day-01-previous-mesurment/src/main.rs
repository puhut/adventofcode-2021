use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let mut previous_number = String::new();
    let mut total = 0;

    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(number) = line {
                if !previous_number.trim().is_empty() {
                    let pN = i32::from_str(&previous_number).unwrap();
                    let oN = i32::from_str(&number).unwrap();
                    if pN < oN {
                        total += 1;
                    }
                }
                previous_number = number;
            }
        }
    }
    println!("total case is {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}