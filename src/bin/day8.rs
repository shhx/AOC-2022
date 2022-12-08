use std::collections::HashMap;

fn main() {
    let input = include_str!("day8_input.txt")
                .lines()
                .map(|l| l.chars()
                .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
                .collect::<Vec<_>>();
    let n = input.len();
    let m = input[0].len();
    let mut visible: HashMap<(usize, usize), u32> = HashMap::new();
    let mut border_length = n * 2 + (m - 2) * 2;
    println!("border_length: {}", border_length);
    println!("n: {}, m: {}", n, m);
    for i in 1..n-1 {
        for j in 1..m-1 {
            let row = &input[i];
            let (fhalf, shalf) = row.split_at(j);
            // println!("Split at {}: {:?}", j, (fhalf, shalf));
            let max_fsplit = fhalf.iter().max().unwrap();
            let max_ssplit = shalf.iter().skip(1).max().unwrap();
            let value = visible.entry((i, j)).or_insert(1);
            *value &= (input[i][j] <= *max_fsplit && input[i][j] <= *max_ssplit) as u32;
            // println!("{} < {:?} && {} < {:?} -> {}", input[i][j], max_fsplit, input[i][j], max_ssplit, value);
            let col = input.iter().map(|r| r[j]).collect::<Vec<_>>();
            let (fhalf, shalf) = col.split_at(i);
            // println!("Split at {}: {:?}", j, (fhalf, shalf));
            let max_fsplit = fhalf.iter().max().unwrap();
            let max_ssplit = shalf.iter().skip(1).max().unwrap();
            *value &= (input[i][j] <= *max_fsplit && input[i][j] <= *max_ssplit) as u32;
            // println!("{} < {:?} && {} < {:?} -> {}", input[i][j], max_fsplit, input[i][j], max_ssplit, value);
            if *value == 0 {
                // println!("{} {} -> {}", i, j, value);
                border_length += 1;
            } 
        }        
    }
    println!("border_length: {}", border_length);
}
