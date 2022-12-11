
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<isize>,
    operation: String,
    test: isize,
    throw_to: [isize; 2],
    inspect_times: usize,
}

fn main() {
    let input = include_str!("day11_input.txt").split("\n\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for group in input {
        // println!("group: {}", group);
        let mut lines = group.lines().skip(1);
        let items = lines.next().unwrap().split_once(":").unwrap();
        let items: Vec<isize> = items.1.split(",").map(|x| x.trim().parse::<isize>().unwrap()).collect();
        println!("items: {:?}", items);
        let operation = lines.next().unwrap().split_once(":").unwrap().1.to_string();
        println!("operation: {}", operation);
        let test = lines.next().unwrap().split_once(":").unwrap().1.trim()
                        .split(" ").nth(2).expect("wrong test").parse::<isize>().unwrap();
        println!("test: {}", test);
        let throw_to1 = lines.next().unwrap().split_once(":").unwrap().1.trim()
                        .split(" ").nth(3).expect("wrong throw_to").parse::<isize>().unwrap();
        let throw_to2 = lines.next().unwrap().split_once(":").unwrap().1.trim()
                        .split(" ").nth(3).expect("wrong throw_to").parse::<isize>().unwrap();
        let throw_to = [throw_to1, throw_to2];
        println!("throw_to: {:?}", throw_to);
        monkeys.push(Monkey { items, operation, test, throw_to, inspect_times: 0 as usize});
    }

    println!();
    println!("Startin rounds!!!");
    for round in 0..20 {
        println!("round: {}", round);
        for m in 0..monkeys.len() {
            let mut monkey = monkeys[m].clone();
            let items_len = monkey.items.len();
            for _ in 0..items_len {
                let mut worry_level = monkey.items.remove(0);
                let operation = &monkey.operation.split_once("=").unwrap();
                // println!("operation: {:?}", operation.1.trim());
                if operation.1.trim().starts_with("old + ") {
                    let num = operation.1.trim().split_once("+")
                                            .unwrap().1.trim().parse::<isize>().unwrap();
                    worry_level += num;

                } else if operation.1.trim().starts_with("old * old") {
                    worry_level *= worry_level;

                } else if operation.1.trim().starts_with("old *") {
                    let num = operation.1.trim().split_once("*")
                                         .unwrap().1.trim().parse::<isize>().unwrap();
                    worry_level *= num;
                } else {
                    unreachable!();
                }
                worry_level = worry_level / 3;

                if worry_level % monkey.test == 0 {
                    monkeys[monkey.throw_to[0] as usize].items.push(worry_level);
                    println!("monkey {} threw {} to {}", m, worry_level, monkey.throw_to[0]);
                } else {
                    monkeys[monkey.throw_to[1] as usize].items.push(worry_level);
                    println!("monkey {} threw {} to {}", m, worry_level, monkey.throw_to[1]);
                }
                monkey.inspect_times += 1;

            }
            monkeys[m] = monkey;
        }
        println!("Round {} done!", round);
        println!("monkeys: {:?}", monkeys);
        let mut times = monkeys.iter().map(|x| x.inspect_times).collect::<Vec<usize>>();
        times.sort();
        let biggest_two = times.iter().rev().take(2).fold(1, |acc, x| acc * x);
        println!("biggest_two: {}", biggest_two);
    }
}
