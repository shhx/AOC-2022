use std::collections::HashSet;
use std::fmt;

#[derive (Debug, Eq, PartialEq, PartialOrd, 
          Ord, Hash, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn distance(self, pos: &Pos) -> f64 {
        let t = self.x.abs_diff(pos.x).pow(2) + self.y.abs_diff(pos.y).pow(2);
        return (t as f64).sqrt()
    }

    fn get_dir(self, pos: &Pos) -> (isize, isize) {
        let dirx = (pos.x - self.x).signum();
        let diry = (pos.y - self.y).signum();
        return  (dirx, diry);
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    let mut seen: HashSet<Pos> = HashSet::new();
    const TAIL_LENGTH: usize = 10;
    let mut tail = [Pos{ x: 0, y:0 }; TAIL_LENGTH];
    for (dir, amount) in input {
        for _ in 0..amount {
            mov(&mut tail[0], dir, 1);
            let mut pos_head = tail[0];
            // println!("True head: {} dir: {} -> {}", pos_head, dir, amount);
            for knot in 1..TAIL_LENGTH {
                let mut deez_nut = &mut tail[knot];
                let mut dist = deez_nut.distance(&pos_head);
                // println!("Knot {} Head:{} Me:{}  dist: {}", knot, pos_head, deez_nut, dist);
                if dist > 1.5 {
                    let (dirx, diry) = deez_nut.get_dir(&pos_head);
                    // println!("Sum ({dirx}, {diry}) to {}", deez_nut);
                    deez_nut.x += dirx;
                    deez_nut.y += diry;
                }

                dist = deez_nut.distance(&pos_head);
                if dist > 1.5 {
                    mov(&mut deez_nut, dir, 1);
                    // dist = deez_nut.distance(pos_head);
                }
                if knot == TAIL_LENGTH - 1 {
                    seen.insert(*deez_nut);
                }
                // println!("Knot {} final pos {}", knot, deez_nut);
                // println!("Seen: {}", seen);
                pos_head = *deez_nut;
                
            }
            // println!();
        }
    }
    println!("True head pos: {}", tail[TAIL_LENGTH - 1]);
    println!("Len seen: {:?} {}", seen, seen.len());
}
