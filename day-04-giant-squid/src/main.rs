use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct Point {
    number: i32,
    called: bool,
}

const ROW : usize = 5;
const COLUMN : usize = 5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);


    let drawn_numbers = [17,58,52,49,72,33,55,73,27,69,88,80,9,7,59,98,63,42,84,37,87,28,97,66,79,77,61,48,83,5,94,26,70,12,51,82,99,45,22,64,10,78,13,18,15,39,8,30,68,65,40,21,6,86,90,29,60,4,38,3,43,93,44,50,41,96,20,62,19,91,23,36,47,92,76,31,67,11,0,56,95,85,35,16,2,14,75,53,1,57,81,46,71,54,24,74,89,32,25,34];
    let mut boards: Vec<[[Point;ROW];COLUMN]> = Vec::new();
    let mut board : [[Point;ROW]; COLUMN] = [[Point {number:0, called: false}; ROW]; COLUMN];
    
    if let Ok(lines) =read_lines(filename){
        let mut row = 0;
        for line in lines {
            if let Ok(numbers) = line {
                if numbers.trim().len() > 0 {
                    for (col, num) in numbers.to_string().split_whitespace().enumerate() {
                        let p : Point = Point { number: num.parse::<i32>().unwrap(), called: false };
                        board[row][col] = p;
                    }
                    row += 1;
                }
            }

            if row == 5 {
                boards.push(board);
                row = 0;
                board = [[Point {number:0, called: false}; ROW]; COLUMN];
            }
        }
    }

    //using drawn_numbers to mark
    //and each mark, need to check if someone wins a game
    'outer: for drawn_num in drawn_numbers {
        for board in boards.iter_mut() {
            for row in 0..ROW {
                for col in 0..COLUMN{
                    if board[row][col].number == drawn_num {
                        board[row][col].called = true;
                    }
                }
            }
            if check_game(*board) {
                let score_num = score(*board, drawn_num);
                println!("you did it: score {} {}", score_num, drawn_num);
                break 'outer;
            }
        }
    }
}
fn check_game(board: [[Point;ROW]; COLUMN]) -> bool {
    for num in 0..ROW{
        if board[num][0].called && board[num][1].called && board[num][2].called && board[num][3].called && board[num][4].called {
            return true;
        } else if board[0][num].called && board[1][num].called && board[2][num].called && board[3][num].called && board[4][num].called {
            return true;
        }
    }
    false
}

fn score(board: [[Point;ROW]; COLUMN], called_number: i32) -> i32 {
    let mut points = 0;
    for col in 0..COLUMN {
        for row in 0..ROW {
            if board[row][col].called == false {
                points += board[row][col].number;
            }
        }
    }
    return points * called_number;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}