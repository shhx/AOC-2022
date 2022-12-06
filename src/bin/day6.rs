use std::cmp;
use std::collections::HashSet;

fn main() {
    let seq = include_str!("day6_input.txt").chars();
    let mut marker: Vec<char> = Vec::new();
    let marker_size = 14;
    for (i, c) in seq.enumerate() {
        if marker.len() < marker_size {
            marker.push(c);
        } else {
            marker.remove(0);
            marker.push(c);
            let set: HashSet<char> = marker.iter().cloned().collect();
            if set.len() == marker_size {
                println!("Found 4 diff chars in row: {} {}", c, i + 1);
                break;
            }
        }
    }

}