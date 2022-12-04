use std::collections::HashSet;

fn letter_to_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 26 + 1
    } else {
        c as u32 - 'a' as u32 + 1
    }

}

fn main() {
    let input = include_str!("day3_input.txt");
    let mut sum_pri = 0;
    for line in input.lines() {
        let length = line.len();
        // let end: usize = line.chars().map(|c| c.len_utf8()).take(length/2).sum();
        // println!("{} {} {}", line, length, end);
        println!("{} - {}", &line[..length/2], &line[length/2..]);
        let set1: HashSet<char> = line[..length/2].chars().collect();
        let set2: HashSet<char> = line[length/2..].chars().collect();
        let letter = set1.intersection(&set2).take(1).next().unwrap();
        let priority = letter_to_priority(*letter);
        println!("Intersect {} -> {}", letter, priority);
        sum_pri += priority;
    }
    println!("Sum {}", sum_pri);
    
    // Part 2
    let input = include_str!("day3_input.txt");
    let mut sum_pri = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let a = lines.chunks(3);
    for chunk in a {
        let sets: Vec<_> = chunk.iter().map(|s| s.chars()).map(|f| f.collect::<HashSet<char>>()).collect();
        let s = sets.iter().skip(1)
        .fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });
        // let mut s = sets[0].clone();
        // for set in sets{
        //     s = s.intersection(&set).copied().collect();
        // }
        let priority = letter_to_priority(*s.iter().next().unwrap());
        println!("{:?} {}", s, priority);
        sum_pri += priority;
    }
    println!("Sum {}", sum_pri);
}
