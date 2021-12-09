use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

const ROW : usize = 100;
const COLUMN : usize = 100;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut hieghtmap :[[u64; COLUMN]; ROW] = [[0u64; COLUMN]; ROW];
    
    println!("In file {}", filename);

    let mut row = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut column = 0;
            if let Ok(line_of_heightmap) = line {
                for c in line_of_heightmap.chars() {
                    hieghtmap[row][column] = c.to_string().parse::<u64>().unwrap();
                    column += 1;
                }
            }
            row += 1;
        }
    }
    
    //check
    let mut sum = 0;
    let mut check_sum = 0;
    for row in 0..ROW {
        for column in 0..COLUMN {
            if row >= 1 && hieghtmap[row-1][column] > hieghtmap[row][column] {
                check_sum += 1;
            } else if row == 0 {
                check_sum += 1;
            } 
            if row + 1 < ROW && hieghtmap[row+1][column] > hieghtmap[row][column] {
                check_sum += 1;
            } else if row == (ROW -1) {
                check_sum += 1;
            }
            if column >= 1 && hieghtmap[row][column-1] > hieghtmap[row][column] {
                check_sum += 1;
            } else if column == 0 {
                check_sum += 1;
            }
            if column+1 < COLUMN && hieghtmap[row][column+1] > hieghtmap[row][column] {
                check_sum += 1;
            } else if column == (COLUMN-1) {
                check_sum += 1;
            }
            //println!("{} {} -> checkSum{}: {}", row, column, check_sum,hieghtmap[row][column]);
            if check_sum == 4 {
                
                sum += hieghtmap[row][column] + 1;
            }
            check_sum = 0;
        }
    }
    println!("total {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}