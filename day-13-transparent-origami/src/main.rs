#![feature(option_result_contains)]
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

type Point = (usize, usize);

fn parse(file: &str) -> (HashSet::<Point>, Vec<(String, usize)>) {
    let file = File::open(file).expect("File cannot be opened");
    let mut transparent_papaer = HashSet::<Point>::new();
    let mut folds = Vec::<(String, usize)>::new();
    let mut instruction = false;

    for line in io::BufReader::new(file).lines() {
        if line.contains(&"".to_string()){
            instruction = true;
            continue;
        }

        let line = line.unwrap();

        if !instruction {
            let mut coord = line.split(',');
            transparent_papaer.insert((coord.next().unwrap().parse::<usize>().unwrap(), coord.next().unwrap().parse::<usize>().unwrap()));
        } else {
            let mut fold = line.split(' ').nth(2).unwrap().split('=').map(|x| x.to_string());
            let axis = fold.next().unwrap();
            let position = fold.next().unwrap().parse::<usize>().unwrap();
            folds.push((axis, position))
        }
    }

    return (transparent_papaer, folds);
}

fn fold(hash_set: &mut HashSet<Point>, fold: &(String, usize)) {
    let points = hash_set.iter().copied().collect::<Vec<Point>>();
    for point in points {
        match fold.0.as_str() {
            "y" => if point.1 > fold.1 {
                hash_set.remove(&point);
                hash_set.insert((point.0, fold.1-(point.1-fold.1)));
            },
            "x" => if point.0 > fold.1 {
                hash_set.remove(&point);
                hash_set.insert((fold.1-(point.0-fold.1), point.1));
            },
            _ => {}
        }
    }
}

fn solve1(file: &str) -> usize {
    let (mut hash, folds) = parse(file);
    fold(&mut hash, &folds[0]);

    return hash.len();
}

fn solve2(file: &str) {
    let (mut hash, folds) = parse(file);
    for cur_fold in folds {
        fold(&mut hash, &cur_fold);
    }

    let max_x = hash.iter().max_by(|point1, point2| point1.0.cmp(&point2.0)).unwrap().0;
    let max_y = hash.iter().max_by(|point1, point2| point1.1.cmp(&point2.1)).unwrap().1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if hash.contains(&(x,y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

}

fn main() {
    println!("total points: {}", solve1("data.txt"));
    solve2("data.txt");
}