use std::cmp;

fn move_crate(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, n_stacks: usize) {
    let final_length = stacks[from].len().saturating_sub(n_stacks);
    let mut crat = stacks[from].split_off(final_length);
    stacks[to].append(&mut crat);
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        for (i, c) in stack.iter().enumerate() {
            for _ in 0..i {
                print!(" ");
            }
            print!("{} ", c);
        }
        println!("");
    }
}


fn get_top_crates(stacks: &Vec<Vec<char>>) -> Vec<char> {
    let mut top_crates: Vec<char> = Vec::new();
    for stack in stacks {
        if stack.len() > 0 {
            top_crates.push(stack[stack.len() - 1]);
        } else {
            top_crates.push(' ');
        }
    }
    top_crates
}

fn main() {
    let (init, moves) = include_str!("day5_input.txt").split_once("\n\n").expect("Wrong input");
    let len_crates = init.lines().count() - 1;
    let n_stacks: usize = init.lines().nth(len_crates).expect("No count")
                            .trim().split(" ").last().unwrap()
                            .parse::<usize>().expect(" Wrong count") + 1;
    println!("{} {}", len_crates, n_stacks);
    let stack_line: Vec<_> = init.lines().take(len_crates).map(|x| x.chars().collect::<Vec<char>>())
                                // .map(|mut x| x.next().unwrap())
                            //  .map(|x| x.clone().next().unwrap())
                            .collect();
                            //  chars().nth(2).unwrap()
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..n_stacks{
        let v: Vec<char> = Vec::new();
        stacks.push(v);
    }

    for line in stack_line {
        println!("{:?}", line);
        for (i, c) in line.iter().enumerate() {
            if !c.is_alphanumeric() {
                continue;
            }
            
            let offset: i32 = (i as i32 - 1) / 4;
            // println!("c:{} i:{} offset used: {}", c, i, offset);
            stacks[(offset) as usize].insert(0, *c);
        }
    }

    println!("{:?}", stacks);
    let moves = moves.lines().map(|x| x.split(" ").collect::<Vec<_>>()).collect::<Vec<_>>();
    for mov in moves {
        println!("{:?}", mov);
        let many = mov[1].parse::<usize>().expect("Wrong move");
        let from = mov[3].parse::<usize>().expect("Wrong move") - 1;
        let to =   mov[5].parse::<usize>().expect("Wrong move") - 1;
        move_crate(&mut stacks, from, to, many);
        println!("{:?}", stacks);
    }

    let top_crates = get_top_crates(&stacks).into_iter().collect::<String>();
    println!("{}", top_crates);
}
