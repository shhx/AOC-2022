use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn main() {
    let mut calories: Vec<i32> = Vec::new();
    let file = File::open("src/bin/day1_input.txt").expect("File not found");
    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Ok(l) = line {
            if l.is_empty(){
                calories.push(0);
            } else {
                let val: i32 = l.trim().parse().unwrap();
                match calories.last_mut() {
                    Some(last) => *last += val,
                    None => calories.push(val),
                }
            }
        }
    }
    calories.sort();
    for num in calories.iter(){
        println!("{}", num);
    }

    let index = calories.len() - 3;
    let maxs = calories.get(index..).unwrap();
    let sum_top3: i32 = maxs.iter().sum();
    println!("Max is {}", calories.last().unwrap());
    println!("Sum of 3 maximum is {}", sum_top3);
}
