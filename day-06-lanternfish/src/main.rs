use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    const MAX_DAY: i32 = 256;
    println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_of_lanternfish) = line {
                /*
                let mut lanternfish : Vec<u8> = line_of_lanternfish.split(",")
                                                .filter_map(|w| w.parse().ok())
                                                .collect();
                println!("{:?}", lanternfish);
                for day in 1..(MAX_DAY + 1) {
                    let mut nextday_lanternfish : Vec<u8> = Vec::new();
                    for fish in lanternfish {
                        match fish {
                            0 => {
                                nextday_lanternfish.push(6);
                                nextday_lanternfish.push(8);
                            },
                            _ => nextday_lanternfish.push(fish-1),
                        }
                    }
                    lanternfish = nextday_lanternfish;
                    println!("day {}: {}", day, lanternfish.len())
                }
                */

                //better version
                let mut lanternfish_population = [0_u64; 9];
                for tick in line_of_lanternfish.split(",").map(|w| w.parse::<usize>().unwrap()) {
                    lanternfish_population[tick] += 1;
                }

                println!("day {}: {}", 80, count_lanternfish(lanternfish_population, 80));
                println!("day {}: {}", 256, count_lanternfish(lanternfish_population, 256));
            }
        }
    }
}

fn count_lanternfish(mut lanternfish_population: [u64;9], day: usize) -> u64 {
    for _ in 0..day {
        lanternfish_population.rotate_left(1);
        lanternfish_population[6] += lanternfish_population[8];
    }
    lanternfish_population.into_iter().sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}