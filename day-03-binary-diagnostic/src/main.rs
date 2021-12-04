use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
struct Diagnostic {
    zeros: i32,
    ones: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    const SIZE : usize = 12;

    let mut diagnostic_result : [Diagnostic; SIZE] = [Diagnostic { zeros: 0, ones: 0}; SIZE];
    if let Ok(lines) = read_lines(filename){
        for line in lines {
            if let Ok(line_string) = line {
                for (i, c) in line_string.chars().enumerate(){
                    if c == '0' {
                        diagnostic_result[i].zeros += 1;
                    } else {
                        diagnostic_result[i].ones += 1;
                    }
                }
            }
        }
    }
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for bit in diagnostic_result {
        if bit.zeros >= bit.ones  {
            gamma = format!("{}{}", gamma, "0");
            epsilon = format!("{}{}", epsilon, "1");
        } else if bit.zeros < bit.ones {
            gamma = format!("{}{}", gamma, "1");
            epsilon = format!("{}{}", epsilon, "0");
        }
    }
    println!("the result is {} {}", gamma, epsilon);
    println!("the result is {} {}\n the first product is {}\n", 
        isize::from_str_radix(&gamma, 2).unwrap(), 
        isize::from_str_radix(&epsilon, 2).unwrap(),
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
    );

    /* bonus */
    /* oxygen */
    let mut result_of_oxygen = "".to_string();
    let mut diagnostic_result_for_oxygen : [Diagnostic; SIZE] = [Diagnostic { zeros: 0, ones: 0}; SIZE];

    // initializing
    let mut init_oxygen_condidate: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                init_oxygen_condidate.push(value)
            }
        }
    }
    let mut i = 0;
    while init_oxygen_condidate.len() != 1 {
        for candidate in &init_oxygen_condidate {
            if candidate.chars().nth(i).unwrap() == '1' {
                diagnostic_result_for_oxygen[i].ones += 1;
            }else{
                diagnostic_result_for_oxygen[i].zeros += 1;
            }
        }

        if diagnostic_result_for_oxygen[i].ones >= diagnostic_result_for_oxygen[i].zeros {
            result_of_oxygen = format!("{}{}", result_of_oxygen, "1");
        } else {
            result_of_oxygen = format!("{}{}", result_of_oxygen, "0");
        }

        let mut oxygen_condidate_temp: Vec<String> = Vec::new();

        for candidate in init_oxygen_condidate {
            if candidate.chars().nth(i).unwrap() == result_of_oxygen.chars().nth(i).unwrap() {
                oxygen_condidate_temp.push(candidate);
            }
        }
        init_oxygen_condidate = oxygen_condidate_temp;
        i += 1;
    }

    println!("{:?} {:?}", init_oxygen_condidate, result_of_oxygen);
    
    /* co2 */
    let mut result_of_co2 = "".to_string();
    let mut diagnostic_result_for_co2 : [Diagnostic; SIZE] = [Diagnostic { zeros: 0, ones: 0}; SIZE];

    // initializing
    let mut init_co2_condidate: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                init_co2_condidate.push(value)
            }
        }
    }

    i = 0;
    while init_co2_condidate.len() != 1 {
        for candidate in &init_co2_condidate {
            if candidate.chars().nth(i).unwrap() == '1' {
                diagnostic_result_for_co2[i].ones += 1;
            }else{
                diagnostic_result_for_co2[i].zeros += 1;
            }
        }

        if diagnostic_result_for_co2[i].ones >= diagnostic_result_for_co2[i].zeros {
            result_of_co2 = format!("{}{}", result_of_co2, "0");
        } else {
            result_of_co2 = format!("{}{}", result_of_co2, "1");
        }

        let mut co2_condidate_temp: Vec<String> = Vec::new();

        for candidate in init_co2_condidate {
            if candidate.chars().nth(i).unwrap() == result_of_co2.chars().nth(i).unwrap() {
                co2_condidate_temp.push(candidate);
            }
        }
        init_co2_condidate = co2_condidate_temp;
        i += 1;
    }

    println!("{:?} {:?}", init_co2_condidate, result_of_co2);
    let oxygen = init_oxygen_condidate.into_iter().nth(0).unwrap();
    let co2 = init_co2_condidate.into_iter().nth(0).unwrap();
    println!(" {} x {} = {}", 
        isize::from_str_radix(&oxygen, 2).unwrap(),
        isize::from_str_radix(&co2, 2).unwrap(),
        isize::from_str_radix(&oxygen, 2).unwrap() * isize::from_str_radix(&co2, 2).unwrap()
    );
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}