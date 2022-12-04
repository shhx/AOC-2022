use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::fs::File;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PResults {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Play {
    elf: PlayElf,
    me: PlayElf,
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum PlayElf{
    A = 1, // rock
    B = 2, // paper
    C = 3, // scissors
}

fn main() {
    let results: HashMap<Play, PResults> = HashMap::from([
        (Play{elf: PlayElf::A, me: PlayElf::A}, PResults::Draw),
        (Play{elf: PlayElf::A, me: PlayElf::B}, PResults::Win),
        (Play{elf: PlayElf::A, me: PlayElf::C}, PResults::Loss),
        (Play{elf: PlayElf::B, me: PlayElf::B}, PResults::Draw),
        (Play{elf: PlayElf::B, me: PlayElf::A}, PResults::Loss),
        (Play{elf: PlayElf::B, me: PlayElf::C}, PResults::Win),
        (Play{elf: PlayElf::C, me: PlayElf::C}, PResults::Draw),
        (Play{elf: PlayElf::C, me: PlayElf::A}, PResults::Win),
        (Play{elf: PlayElf::C, me: PlayElf::B}, PResults::Loss),
    ]);
    
    let play_map: HashMap<&str, PlayElf> = HashMap::from([
        ("X", PlayElf::A),
        ("Y", PlayElf::B),
        ("Z", PlayElf::C),
        ("A", PlayElf::A),
        ("B", PlayElf::B),
        ("C", PlayElf::C),
    ]);

    let end_results: HashMap<&str, PResults> = HashMap::from([
        ("X", PResults::Loss),
        ("Y", PResults::Draw),
        ("Z", PResults::Win),
    ]);

    // Part 1
    let input = File::open("day2_input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();
    let mut total_score = 0;
    for line in lines {
        if let Ok(l) = line {
            let mut plays = l.trim().split(" ");
            let p1 = plays.next().unwrap();
            let p2 = plays.next().unwrap();
            println!("{} {}", p1, p2);
            let play_s = &Play {
                elf: play_map.get(p1).unwrap().clone(),
                me: play_map.get(p2).unwrap().clone(),
            };
            let score = results.get(play_s).expect("Imposible play");
            total_score += *score as i32;
            total_score += play_s.me as i32;
            println!("{:?}", *score);
            // println!("{:} {:}", *score as i32, play_s.me as i32);
        }
    }
    println!("Total score: {}", total_score);

    // Part 2
    let file = File::open("day2_input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();
    let mut total_score = 0;
    for line in lines {
        if let Ok(l) = line {
            let mut plays = l.trim().split(" ");
            let p1 = plays.next().unwrap();
            let p2 = plays.next().unwrap();
            let result = end_results.get(p2).expect("Impossible result");
            // println!("{:?} {:?} {:?}", play_map.get(p1).unwrap(), play_map.get(p2).unwrap(), result);
            let values: Vec<&Play> = results.iter()
                                .filter_map(|(key, &val)| if val == *result {Some(key)} else {None})
                                .collect();
            let my_play = values.iter().filter(|p| p.elf == play_map.get(p1)
                                .unwrap().clone())
                                .next().expect("No play");
            // println!("{:?}", my_play.me);
            total_score += *result as i32;
            total_score += my_play.me as i32;
            //println!("{:} {:}", *result as i32, my_play.me as i32);
        }
    }
    println!("Total score: {}", total_score);
}
