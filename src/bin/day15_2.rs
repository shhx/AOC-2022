use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn check(closest_beacon: &HashMap<Point, Point>, point: &Point) -> bool {
    let mut break_loop = true;
    for (sensor, beacon) in closest_beacon.iter() {
        if *point == *beacon || point.x < 0 || point.y < 0 
        || point.x > 4000000 || point.y > 4000000{
            break_loop &= false;
            continue;
        }
        // println!("{} {} {}", sensor.distance(&beacon), sensor.distance(&point),
        //                      sensor.distance(&beacon) >= sensor.distance(&point));
        // println!("{:?} {:?} {:?}", point, sensor.distance(&point), sensor.distance(&beacon));
        break_loop &= sensor.distance(&point) > sensor.distance(&beacon);
    }
    return break_loop;
}

fn main() {
    let input = include_str!("day15_input.txt").lines();
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut closest_beacon: HashMap<Point, Point> = HashMap::new();
    for line in input {
        let mut digit: Vec<isize> = Vec::new();
        for cap in re.captures_iter(line){
            digit.push(cap[1].parse::<isize>().unwrap());
        }
        
        let sensor = Point { x: digit[0], y: digit[1] };
        let beacon = Point { x: digit[2], y: digit[3] };
        // println!("{:?} {:?}", beacon, sensor);
        closest_beacon.insert(sensor, beacon);
    }
    let offset = 0;
    for (sensor, beacon) in closest_beacon.iter() {
        // println!("{:?} {:?}", sensor, beacon);
        let distance = sensor.distance(&beacon) as isize + offset;
        for i in 0..distance {
            let x = sensor.x + distance - i;
            let y = sensor.y + i + 1;
            let point = Point { x, y };
            let res = check(&closest_beacon, &point);
            if res {
                println!("{:?} {:?}", point, point.x * 4000000 + point.y);
                panic!();
            }
            // println!("{:?} {:?}", point, point.x * 4000000 + point.y);
        }
        // println!();

        for i in 0..distance + 2 {
            let x = sensor.y - distance + i;
            let y = sensor.x - i - 1;
            let point = Point { x, y };
            let res = check(&closest_beacon, &point);
            if res {
                println!("{:?} {:?}", point, point.x * 4000000 + point.y);
                panic!();
            }

            // println!("{:?} {:?}", point, point.x * 4000000 + point.y);
        }
        // println!();

        for i in 0..distance + 1 {
            let x = sensor.x + distance - i + 1;
            let y = sensor.y - i;
            let point = Point { x, y };
            let res = check(&closest_beacon, &point);
            if res {
                println!("{:?} {:?}", point, point.x * 4000000 + point.y);
                panic!();
            }
            // println!("{:?} {:?}", point, point.x * 4000000 + point.y);
        }
        // println!();

        for i in 0..distance {
            let x = sensor.x - i - 1;
            let y = sensor.y + distance - i;
            let point = Point { x, y };
            let res = check(&closest_beacon, &point);
            if res {
                println!("{:?} {:?}", point, point.x * 4000000 + point.y);
                panic!();
            }

            // println!("{:?}", point);
        }
        // println!();

    }

}


// 32000008000000 too high