use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let buckets : HashMap<char, char> = HashMap::from([
        ('{', '}'),
        ('(', ')'),
        ('[', ']'),
        ('<', '>')
    ]);

    let score : HashMap<char, i32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);

    println!("In file {}", filename);

    let mut result : Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_syntax) = line {
                let mut stack : Vec<char> = Vec::new();
                for c in line_syntax.chars() {
                    if "<{[(".contains(c) {
                        stack.push(c);
                    } else {
                        if buckets[&stack.pop().unwrap()] != c {
                            result.push(score[&c]);
                            break;
                        }
                    }

                }
            }
        }
    }

    println!("{:?}", result.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}