
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rock {
    height: isize,
    width: isize,
    rocks: Vec<Point>,
}

fn get_rocks() -> Vec<Rock> {
    let mut rocks: Vec<Rock> = Vec::new();
    rocks.push(
        Rock { height: 1, width: 3, 
            rocks: vec![
                Point {x: 0, y:0},
                Point {x: 1, y:0},
                Point {x: 2, y:0},
                Point {x: 3, y:0},
            ]}
    );
    rocks.push(
        Rock { height: 3, width: 3, 
            rocks: vec![
                Point {x: 0, y:1},
                Point {x: 1, y:1},
                Point {x: 1, y:2},
                Point {x: 1, y:0},
                Point {x: 2, y:1},
            ]}
    );
    rocks.push(
        Rock { height: 3, width: 3, 
            rocks: vec![
                Point {x: 0, y:0},
                Point {x: 1, y:0},
                Point {x: 2, y:0},
                Point {x: 2, y:1},
                Point {x: 2, y:2},
            ]}
    );
    rocks.push(
        Rock { height: 4, width: 1, 
            rocks: vec![
                Point {x: 0, y:0},
                Point {x: 0, y:1},
                Point {x: 0, y:2},
                Point {x: 0, y:3},
            ]}
    );
    rocks.push(
        Rock { height: 2, width: 2, 
            rocks: vec![
                Point {x: 0, y:0},
                Point {x: 1, y:0},
                Point {x: 0, y:1},
                Point {x: 1, y:1},
            ]}
    );
    return rocks;
}

fn print_cave(cave: &Vec<[&str; 7]>, max_height: usize) {
    for y in (0..max_height).rev() {
        for x in 0..7 {
            print!("{}", cave[y][x]);
        }
        println!("");
    }
    println!();
}

impl Rock {
    fn touches(&self, cave: &Vec<[&str; 7]>, offset_x:isize, offset_y: isize) -> bool {
        for p in &self.rocks {
            if p.y + offset_y < 0 || p.x + offset_x < 0 || p.x + offset_x >= 7{
                return true;
            }

            if cave[(p.y + offset_y) as usize][(p.x + offset_x) as usize] == "#" {
                return true;
            }
        }
        return false;
    }

    fn set(&self, cave: &mut Vec<[&str; 7]>, offset_x: isize, offset_y: isize) {
        for p in &self.rocks {
            cave[(p.y + offset_y) as usize][(p.x + offset_x) as usize] = "#";
        }
    }
}

const MAX_LINES: usize = 10000;

fn main() {
    let input = include_str!("day17_input.txt").lines().nth(0)
                .map(|x| x.chars().collect::<Vec<_>>()).unwrap();
    println!("{:?}", input);

    let mut cave = vec![["."; 7]; MAX_LINES];
    let rocks = get_rocks();
    
    let mut rock_cnt = 0;
    let mut top_pos: isize = 0;
    let mut counter = 0;
    loop {
        let next_rock = &rocks[rock_cnt % rocks.len()];
        // println!("{:?}", next_rock);
        let cave_h = cave.len() as isize;
        let mut y_pos = top_pos + 3;
        let mut x_pos = 2;
        loop {
            if top_pos > cave_h {
                panic!("Cave too small");
            }
            let stream = input[counter];
            // print!("{}", stream);
            counter = (counter + 1) % input.len();
            let dir = if stream == '<' { -1 } else { 1 };
            x_pos += dir;
            // println!("Strm: x: {}, y: {}", x_pos, y_pos);
            if next_rock.touches(&cave, x_pos, y_pos) {
                x_pos -= dir;
                // println!("touching stream");
            }
            
            y_pos -= 1;
            // println!("Fall: x: {}, y: {}", x_pos, y_pos);
            if next_rock.touches(&cave, x_pos, y_pos) {
                y_pos += 1;
                next_rock.set(&mut cave, x_pos, y_pos);

                if y_pos + next_rock.height > top_pos {
                    top_pos = y_pos + next_rock.height;
                }
                // for i in top_pos..top_pos + 10 {
                //     for j in 0..7 {
                //         if cave[i as usize][j] == "#" {
                //             top_pos = cmp::max(top_pos, i);
                //         }
                //     }
                // }
                // println!("touching, top_pos: {}", top_pos);
                // println!("Set: x: {}, y: {}", x_pos, y_pos);
                break;
            } 
        }
        rock_cnt += 1;
        if rock_cnt > 2021 {
            println!("{}, {}", top_pos, rock_cnt);
            break;
        }
        // print_cave(&cave, top_pos as usize);
    }
}

// 2885 too low
// 3135 too low