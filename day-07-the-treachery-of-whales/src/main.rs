use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut position_sum_board: HashMap<i32, i32> = HashMap::new();
    println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_of_crabs) = line {
                let position_of_crabs : Vec<i32> = line_of_crabs.split(",").filter_map(|x| x.parse().ok()).collect();
                println!("{:?}", position_of_crabs);

                for p1 in &position_of_crabs {
                    let mut sum = 0;
                    for p2 in &position_of_crabs {
                        sum += i32::abs(p1-p2);
                    }
                    position_sum_board.insert(*p1, sum);
                }
                /* bonus
                let position_of_crabs : Vec<i32> = line_of_crabs.split(",").filter_map(|x| x.parse().ok()).collect();
                for p1 in 0..1856 {
                    let mut sum = 0;
                    for p2 in &position_of_crabs {
                        let v = i32::abs(p1-p2);
                        let result = v*(v+1)/2;
                        sum += result;
                    }
                    position_sum_board.insert(p1, sum);
                }
                */
            }
        }
    }
    println!("{:?}", position_sum_board);
    println!("{:?}", position_sum_board.values().min());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}