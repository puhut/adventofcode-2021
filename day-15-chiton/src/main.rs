use pathfinding::directed::dijkstra;

const NEXT: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

//part1
/*
fn main() {
    let map: Vec<Vec<_>> = include_str!("../data.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c- b'0').collect())
        .collect();
    let goal = (map[0].len() as i32 - 1, map.len() as i32 - 1);

    println!("{}",
        dijkstra::dijkstra(
            &(0,0),
            |(x, y)| {
                NEXT.iter()
                    .map(|(xx, yy)| {
                        map.get((y + yy) as usize)
                            .and_then(|r| r.get((x + xx) as usize))
                            .map(|c| ((x + xx, y + yy), *c as u32))
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            },
            |&p| p == goal,
        
        )
        .unwrap()
        .1,
    );
}
*/
//part2
fn main() {
    let map: Vec<Vec<_>> = include_str!("../data.txt")
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect();
    let s = map.len();
    let goal = (s as i32 * 5 - 1, s as i32 * 5 - 1);

    println!(
        "{}",
        dijkstra::dijkstra(
            &(0, 0),
            |&(x, y)| {
                NEXT.iter()
                    .map(|&(xx, yy)| ((x + xx) as usize, (y +yy) as usize))
                    .filter(|(x, y)| (x /5 < s && y /5 < s))
                    .map(|(x, y)| {
                        map.get(y % s)
                        .and_then(|r| r.get(x % s))
                        .map(|c| {
                            (
                                (x as i32, y as i32),
                                ((*c as usize + (x/s) + (y/s) - 1) % 9 + 1) as u32,
                            )
                        })
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            },
            |&p| p == goal,
        )
        .unwrap()
        .1,
    );
}
