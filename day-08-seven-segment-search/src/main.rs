use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum = 0;
    println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_signal) = line {
                let mut items = line_signal.split("|");
                let mut analyasis : Vec<&str> = items.nth(0).unwrap().split_whitespace().collect();
                let mut current_string_pattern : HashMap<String, char> = HashMap::new();
                analyasis.sort_by_key(|name| name.len());
                println!("{:?}", analyasis);
                current_string_pattern.insert(analyasis[0].chars().sorted().collect::<String>(), '1');
                current_string_pattern.insert(analyasis[1].chars().sorted().collect::<String>(), '7');
                current_string_pattern.insert(analyasis[2].chars().sorted().collect::<String>(), '4');
                current_string_pattern.insert(analyasis[3].chars().sorted().collect::<String>(), compare_len5(analyasis[3].chars().sorted().collect::<String>(), analyasis[0]));
                current_string_pattern.insert(analyasis[4].chars().sorted().collect::<String>(), compare_len5(analyasis[4].chars().sorted().collect::<String>(), analyasis[0]));
                current_string_pattern.insert(analyasis[5].chars().sorted().collect::<String>(), compare_len5(analyasis[5].chars().sorted().collect::<String>(), analyasis[0]));
                current_string_pattern.insert(analyasis[6].chars().sorted().collect::<String>(), compare_len6(analyasis[6].chars().sorted().collect::<String>(), analyasis[0]));
                current_string_pattern.insert(analyasis[7].chars().sorted().collect::<String>(), compare_len6(analyasis[7].chars().sorted().collect::<String>(), analyasis[0]));
                current_string_pattern.insert(analyasis[8].chars().sorted().collect::<String>(), compare_len6(analyasis[8].chars().sorted().collect::<String>(), analyasis[0]));
                current_string_pattern.insert(analyasis[9].chars().sorted().collect::<String>(), '8');  

                let mut num_string = "".to_owned();

                let mut problem : Vec<&str> = items.nth(0).unwrap().trim().split_whitespace().collect();
                for segment in problem {
                    num_string = format!("{}{}", num_string, current_string_pattern.get(&segment.chars().sorted().collect::<String>()).unwrap());
                }
                println!("{:?}", current_string_pattern);
                println!("{}", num_string);
                sum += num_string.parse::<i32>().unwrap();
/*
                for segment in items.nth(0).unwrap().trim().split_whitespace() {
                    num_string = format!("{}{}", num_string, current_string_pattern.get(segment).unwrap());
                }
                println!("{} : {}", items.nth(1).unwrap(), num_string);
  */              
                /* init
                for segment in line_signal.split("|").nth(1).unwrap().split_whitespace() {
                    match segment.len() {
                        2 => *result.entry(1).or_insert(0) += 1,
                        3 => *result.entry(7).or_insert(0) += 1,
                        4 => *result.entry(4).or_insert(0) += 1,
                        7 => *result.entry(8).or_insert(0) += 1,
                        _ => (),
                    }
                }*/
            }
        }
    }
    println!("total {}", sum);
    /*
    let mut sum = 0;
    for x in result.values() {
        sum += x;
    }
    println!("total: {}", sum);
    */
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compare_len5(origin: String, first: &str) -> char {
    if origin.contains(first.chars().nth(0).unwrap()) && origin.contains(first.chars().nth(1).unwrap()) {
        return '3';
    } else if origin.contains(first.chars().nth(0).unwrap()) && !origin.contains(first.chars().nth(1).unwrap()) {
        return '2';
    } else if origin.contains(first.chars().nth(1).unwrap()) && !origin.contains(first.chars().nth(0).unwrap()) {
        return '5';
    }
    return '0';
}

fn compare_len6(origin: String, first: &str) -> char {
    if origin.contains(first.chars().nth(0).unwrap()) && origin.contains(first.chars().nth(1).unwrap()) {
        return '9';
    } else {
        return '6';
    }
}