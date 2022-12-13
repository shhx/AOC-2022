
fn print_screen(screen: &Vec<Vec<char>>, lower_limit: usize) {
    let limits_x = [480, 510];
    
    println!("");
    for j in 0..lower_limit + 5 {
        for i in limits_x[0]..limits_x[1]{
            print!("{}", screen[j][i]);
        }
        print!(" {}", j);
        println!("");
    }
}

fn main() {
    let input = include_str!("day14_input.txt").lines();
    let limits = (200, 1000);
    let mut screen = vec![vec!['.']];
    screen.resize(limits.0, vec!['.']);
    for row in screen.iter_mut() {
        row.resize(limits.1, '.');
    }
    let mut lower_limit = 0;
    for l in input {
        let groups: Vec<_> = l.split("->").collect();
        for group in groups.windows(2) {
            // println!("{}", rock);
            let from = group[0];
            let to = group[1];
            let from: Vec<_> = from.split(",").map(|x| x.trim().parse::<usize>().expect("not number")).collect();
            let to: Vec<_> = to.split(",").map(|x| x.trim().parse::<usize>().expect("not number")).collect();
            let from = (from[0], from[1]);
            let to = (to[0], to[1]);
            // println!("{:?} -> {:?}", from, to);
            if to.1 > lower_limit {
                lower_limit = to.1;
            }
            // COpilot made up shit
            if from.0 == to.0 {
                // vertical
                let (x, y) = from;
                let (x2, y2) = to;
                if y < y2 {
                    for y in y..=y2 {
                        screen[y][x] = '#';
                    }
                } else {
                    for y in y2..=y {
                        screen[y][x] = '#';
                    }
                }
            } else {
                // horizontal
                let (x, y) = from;
                let (x2, y2) = to;
                if x < x2 {
                    for x in x..=x2 {
                        screen[y][x] = '#';
                    }
                } else {
                    for x in x2..=x {
                        screen[y][x] = '#';
                    }
                }
            }
        }
    }
    lower_limit += 2;
    // print_screen(&screen);
    let start = (500, 0);
    let mut sand_count = 0;
    'BIG_LOOP: loop {
        let mut sand_point = start;
        loop {
            let next_pos = check_bounds(sand_point.0, sand_point.1, &screen);
            if screen[start.1][start.0] == 'o' {
                break 'BIG_LOOP;
            }
            if next_pos == (0, 0) || next_pos.1 == lower_limit {
                screen[sand_point.1][sand_point.0] = 'o';
                sand_count += 1;
                break;
            }
            // println!("next_pos: {:?}", next_pos);
            sand_point = next_pos;
            // println!("sand_point: {:?}", sand_point);
        }
        // println!("sand_count: {}", sand_count);
        // print_screen(&screen, lower_limit);
    }

    print_screen(&screen, lower_limit);
    println!("sand_count: {}", sand_count);
    println!("lower_limit: {}", lower_limit);
}

fn check_bounds(x: usize, y: usize, screen: &Vec<Vec<char>>) -> (usize, usize) {
    if screen[y + 1][x] == '.' {
        return (x, y + 1);
    } else if screen[y + 1][x - 1] == '.' {
        return (x - 1, y + 1);
    } else if screen[y + 1][x + 1] == '.' {
        return (x + 1, y + 1);
    } else {
        return (0, 0);
    }
}
