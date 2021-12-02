use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(string_instruction) = line {
                let instruction : Vec<&str> = string_instruction.split_whitespace().collect();
            
                let command = instruction[0];
                let num : i32 = instruction[1].trim().parse().unwrap();
                match command {
                    "forward" => {
                        horizontal_position += num;
                        depth += aim * num;
                    },
                    "down" => aim += num,
                    "up" => aim -= num,
                    _ => println!("wrong"),
                }
            }
            
        }
    }
    println!("your product of horizontal and depth is {}", horizontal_position * depth);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}