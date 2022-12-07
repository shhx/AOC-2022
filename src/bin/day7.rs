use std::collections::HashMap;

fn parse_cmd(dirs: &mut HashMap<String, usize>, line: &Vec<&str>, last_dir: &mut Vec<String>){
    let cmd = line[1];
    println!("CMD: {:?}, last: {:?}", line, last_dir);
    println!("dirs: {:?}", dirs);
    match cmd {
        "cd" => {
            let path = line[2];
            if path == ".." {
                last_dir.pop();
                return;
            }
            last_dir.push(path.to_string());
        },
        _ => (),
    }
}

fn main() {
    let input = include_str!("day7_input.txt").lines();
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut last_dir: Vec<String> = vec!["".to_string()];
    for line in input.skip(1) {
        let words = line.split(" ").collect::<Vec<&str>>();
        match words[0] {
            "$" => {
                parse_cmd(&mut dirs, &words, &mut last_dir);
                println!("CMD: {}", line);
            }

            "dir" => println!("Dir: {}", line),
            _ => {
                let mut abs_path = "".to_string();
                for d in last_dir.iter() {
                    abs_path += &d.clone();
                    abs_path += "/";
                    let key = abs_path.clone();
                    let dir = dirs.entry(key.clone()).or_insert(0);
                    *dir += words.first().expect("not a file").parse::<usize>().expect("Not a number");
                    println!("File: {}, {} -> {}", line, key, dir);
                }
            }
        }
    }
    
    println!("dirs: {:?}", dirs);
    let limit = 100000;
    let sum: usize = dirs.iter().filter(|(_, v)| **v <= limit).fold(0, |acc, (_, v)| acc + v);
    println!("{}", sum);
}
