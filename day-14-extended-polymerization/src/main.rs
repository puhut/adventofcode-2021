use std::collections::BTreeMap;
use std::fs::read_to_string;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn calculate_frequencies(input: &str, steps: usize) -> usize {
    let (template, insertions) = input.split_once("\n\n").unwrap();

    let insertions = insertions
                        .lines()
                        .filter_map(|line| line.split_once(" -> "))
                        .map(|(pair, insertion)|{
                            (
                                (pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()),
                                insertion.chars().next().unwrap(),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
    
    let frequncies = template.chars().tuple_windows().map(|(a,b)| (a, b)).counts();

    let x = (0..steps).fold(frequncies, |frequncies, _| {
        let mut new_frequencies = frequncies.clone();
        for pair @((a, b), f) in frequncies {
            if let Some(insertion) = insertions.get(&pair.0) {
                *new_frequencies.entry((a, *insertion)).or_insert(0) += f;
                *new_frequencies.entry((*insertion, b)).or_insert(0) += f;
                *new_frequencies.entry((a, b)).or_insert(0) -= f;
            }
        }
        new_frequencies
    });

    let mut letter_frequencies = BTreeMap::new();

    for ((a, b), f) in x {
        letter_frequencies.entry(a).or_insert((0, 0)).0 += f;
        letter_frequencies.entry(b).or_insert((0, 0)).1 += f;
    }

    if let MinMax(min, max) = letter_frequencies.into_iter().map(|( _, (l, r))| l.max(r)).minmax() {
        max - min
    } else {
        unreachable!()
    }


}

fn main() {
    let input = read_to_string("data.txt").expect("input file not found");
    let now = std::time::Instant::now();
    println!("PART 1 = {}", calculate_frequencies(&input, 10));
    println!("PART 2 = {}", calculate_frequencies(&input, 40));
    println!("Execution time: {:?}", now.elapsed());
}
