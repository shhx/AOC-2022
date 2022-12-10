
fn check_sprite(x: isize, pixel_pos: usize, row: usize, screen: &mut [[char; 40]; 6]) {
    println!("x: {} pix pos:{}, row:{}", x, pixel_pos, row);
    if (x-1..=x+1).contains(&(pixel_pos as isize)) {
        screen[row][pixel_pos] = '#';
    }
}

fn main() {
    let input = include_str!("day10_input.txt").lines();
    let mut cycle_count: usize = 1;
    let mut x: isize = 1;
    let mut screen = [['.'; 40]; 6];

    // x is the sprite position
    let pixel_pos = (cycle_count - 1) % 40;
    cycle_count += 1;
    let row = (cycle_count - 1) / 40;
    check_sprite(x, pixel_pos, row, &mut screen);
    for line in input {
        let mut split = line.split(" ");
        let mut pixel_pos = 0;
        let mut row = 0;
        // println!("cycle_count: {}", cycle_count);
        match split.next().unwrap() {
            "noop" => {
                pixel_pos = (cycle_count - 1) % 40;
                cycle_count += 1;
                row = (cycle_count - 1) / 40;
                check_sprite(x, pixel_pos, row, &mut screen);
            },
            "addx" => {
                let value = split.next().unwrap().parse::<isize>().unwrap();
                pixel_pos = (cycle_count - 1) % 40;
                cycle_count += 1;
                row = (cycle_count - 1) / 40;
                check_sprite(x, pixel_pos, row, &mut screen);
                
                x += value;
                pixel_pos = (cycle_count - 1) % 40;
                cycle_count += 1;
                if cycle_count >= 240 {
                    break;
                }
                row = (cycle_count - 1) / 40;
                check_sprite(x, pixel_pos, row, &mut screen);
            },
            _ => unreachable!(),
        }
        println!("cycle_count: {}", cycle_count);
        if cycle_count >= 240 {
            break;
        }

    }
    for row in screen {
        for pixel in row {
            print!("{}", pixel);
        }
        println!();
    }
}
