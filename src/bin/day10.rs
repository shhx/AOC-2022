
fn check_cycle_count(cycle_count: usize) -> bool {
    if (cycle_count as isize - 20) % 40 == 0 {
        return true;
    }
    return false;
}

fn main() {
    let input = include_str!("day10_input.txt").lines();
    let mut cycle_count: usize = 1;
    let mut x: isize = 1;
    let mut sum = 0;
    for line in input {
        let mut split = line.split(" ");
        match split.next().unwrap() {
            "noop" => {
                cycle_count += 1;
                if check_cycle_count(cycle_count) {
                    sum += x * cycle_count as isize;
                }
            },
            "addx" => {
                let value = split.next().unwrap().parse::<isize>().unwrap();
                cycle_count += 1;
                
                if check_cycle_count(cycle_count) {
                    println!("x: {}", x);
                    sum += x * cycle_count as isize;
                }
                x += value;
                cycle_count += 1;
                if check_cycle_count(cycle_count) {
                    println!("x: {}", x);
                    sum += x * cycle_count as isize;
                }
            },
            _ => unreachable!(),
        }
        // println!("cycle_count: {}", cycle_count);
        if cycle_count > 240  {
            break;
        }
    }
    println!("{}", sum);
}
