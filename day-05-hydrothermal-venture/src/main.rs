use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut scores: HashMap<Point, i32> = HashMap::new();

    let filename = &args[1];
    println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_of_vents) = line {
                let points : Vec<&str> = line_of_vents.split("->").collect();
                
                let x1y1_string : Vec<&str> = points[0].trim().split(",").collect();
                let x1y1 : Point = Point {x: x1y1_string[0].parse::<i32>().unwrap(), y: x1y1_string[1].parse::<i32>().unwrap()};

                let x2y2_string : Vec<&str> = points[1].trim().split(",").collect();
                let x2y2 : Point = Point {x: x2y2_string[0].parse::<i32>().unwrap(), y: x2y2_string[1].parse::<i32>().unwrap()};

                if x1y1.x == x2y2.x && x1y1.y>=x2y2.y {
                    for y in (x2y2.y)..(x1y1.y+1) {
                        let point = Point{ x: x1y1.x, y: y};
                        if !scores.contains_key(&point) {
                            scores.insert(point, 1);
                        } else {
                            scores.insert(point, scores.get(&point).unwrap_or(&0)+1 );
                        }
                    }
                } else if x1y1.x == x2y2.x && x1y1.y < x2y2.y{
                    for y in (x1y1.y)..(x2y2.y+1) {
                        let point = Point{ x: x1y1.x, y: y};
                        if !scores.contains_key(&point) {
                            scores.insert(point, 1);
                        } else {
                            scores.insert(point, scores.get(&point).unwrap_or(&0)+1 );
                        }
                    }
                } else if x1y1.y == x2y2.y && x1y1.x >= x2y2.x {
                    for x in (x2y2.x)..(x1y1.x+1) {
                        let point = Point{ x: x, y: x1y1.y};
                        if !scores.contains_key(&point) {
                            scores.insert(point, 1);
                        } else {
                            scores.insert(point, scores.get(&point).unwrap_or(&0)+1 );
                        }
                    }
                } else if x1y1.y == x2y2.y && x1y1.x < x2y2.x {
                    for x in (x1y1.x)..(x2y2.x+1) {
                        let point = Point{ x: x, y: x1y1.y};
                        if !scores.contains_key(&point) {
                            scores.insert(point, 1);
                        } else {
                            scores.insert(point, scores.get(&point).unwrap_or(&0)+1 );
                        }
                    }
                }
            }
        }
    }
    let mut counter = 0;
    for (key, value) in scores.iter() {
        if value > &1 {
            counter += 1
        }
    }
    println!( "here you go {}", counter);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}