use std::collections::HashSet;

#[derive (Debug, Eq, PartialEq, PartialOrd, 
          Ord, Hash, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn distance(self, pos: Pos) -> f64{
        let t = self.x.abs_diff(pos.x).pow(2) + self.y.abs_diff(pos.y).pow(2);
        return (t as f64).sqrt()
    }
}


fn mov(pos: &mut Pos, dir: &str, amount: isize) {
    match dir {
        "R" => pos.x += amount,
        "L" => pos.x -= amount,
        "U" => pos.y += amount,
        "D" => pos.y -= amount,
        _ => unreachable!(),
    }
}


fn main() {
    let input = include_str!("day9_input.txt")
                .lines()
                .map(|l| l.split_once(" ").unwrap())
                .map(|(d, v)| (d, v.parse::<isize>().unwrap()))
                .collect::<Vec<_>>();
    let mut pos_head = Pos{ x: 0, y:0 };
    let mut pos_tail = Pos{ x: 0, y:0 };
    let mut seen: HashSet<Pos> = HashSet::new();
    for (dir, amount) in input {
        mov(&mut pos_head, dir, amount);
        println!("Pos head: {:?} dir: {} -> {}", pos_head, dir, amount);

        let mut dist = pos_tail.distance(pos_head);
        if dist > 1.5 {
            if dir == "U" || dir == "D" && pos_head.x != pos_tail.x {
                pos_tail.x += pos_head.x - pos_tail.x;
            } else if dir == "L" || dir == "R" && pos_head.y != pos_tail.y {
                pos_tail.y += pos_head.y - pos_tail.y;
            }
            // println!("Pos tail after: {:?}", pos_tail);
        }


        while dist > 1.5 {
            mov(&mut pos_tail, dir, 1);
            dist = pos_tail.distance(pos_head);
            println!("{}", dist);
            seen.insert(pos_tail);
            // println!("Pos tail: {:?}", pos_tail);
        }
        // println!("Seen: {:?}", seen);
        // println!();
    }
    println!("Head pos: {:?}", pos_tail);
    println!("Len seen: {}", seen.len() + 1);
}
