use std::collections::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    count: isize,
    pos: (isize, isize),
}
fn main() {
    let input: Vec<_> = include_str!("day12_input.txt")
                        .lines()
                        .map(|x| x.chars().collect::<Vec<_>>()).collect();
    let n: usize = input.len();
    let m: usize = input[0].len();
    let mut map: Vec<Vec<isize>> = vec![Vec::new(); n];
    let mut destination: (isize, isize) = (0, 0);
    let mut start: (isize, isize) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            let value = input[i][j];
            match value {
                'S' => {
                    map[i].push((b'a' - b'a') as isize);
                    start = (i as isize, j as isize);
                }
                'E' => {
                    destination = (i as isize, j as isize);
                    map[i].push((b'z' - b'a') as isize);
                }
                _ => map[i].push((value as u8 - b'a') as isize)
            }
        }
    }

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut to_visit: Vec<Node> = Vec::new();
    let n_start = Node { count: 0, pos: start };
    visited.insert(n_start.pos);
    to_visit.push(n_start);
    while !to_visit.is_empty() {
        let next = to_visit.remove(0);
        let count = next.count + 1;
        for (i, j) in vec![(next.pos.0 + 1, next.pos.1), (next.pos.0, next.pos.1 + 1), 
                           (next.pos.0 - 1, next.pos.1), (next.pos.0, next.pos.1 - 1)] {
            if i >= n as isize || j >= m as isize || i < 0 || j < 0 {
                continue;
            }
            if visited.contains(&(i, j)) {
                continue;
            }
            let diff = map[i as usize][j as usize] - map[next.pos.0 as usize][next.pos.1 as usize];
            // println!("{} {} -> {}", i, j, diff);
            if diff <= 1 {
                let node = Node { count, pos: (i, j) };
                // println!("In ({:?}) val: {} -> checking ({},{})", next, map[next.pos.0 as usize][next.pos.1 as usize], i, j);
                to_visit.push(node);
                visited.insert(node.pos);
            }
            if (i, j) == destination && diff <= 1 {
                println!("Found destination in {} steps", count);
                return;
            }
        }
    }
}
