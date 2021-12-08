use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut result: HashMap<i32, i32> = HashMap::new();
    println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_signal) = line {
                for segment in line_signal.split("|").nth(1).unwrap().split_whitespace() {
                    match segment.len() {
                        2 => *result.entry(1).or_insert(0) += 1,
                        3 => *result.entry(7).or_insert(0) += 1,
                        4 => *result.entry(4).or_insert(0) += 1,
                        7 => *result.entry(8).or_insert(0) += 1,
                        _ => (),
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for x in result.values() {
        sum += x;
    }
    println!("total: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}