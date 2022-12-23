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

fn check(closest_beacon: &HashMap<Point, Point>, point: &Point, not_available: &mut usize) -> bool {
    let mut break_loop = true;
    for (sensor, beacon) in closest_beacon.iter() {
        if *point == *beacon {
            // println!("equals {:?} {:?} {:?}", sensor, beacon, point);
            break_loop = false;
            continue;
        }
        // println!("{:?} {:?} {:?}", sensor, beacon, point);
        // println!("{} {} {}", sensor.distance(&beacon), sensor.distance(&point),
        //                      sensor.distance(&beacon) >= sensor.distance(&point));
        if sensor.distance(&beacon) >= sensor.distance(&point) {
            *not_available += 1;
            break_loop = false;
            break;
        }
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

    let mut point = Point { x: 0, y: 2000000 };
    let mut not_available = 0;
    loop {
        if check(&closest_beacon, &point, &mut not_available) {
            break;
        }
        point.x += 1;
    }
    point.x = -1;
    loop {
        if check(&closest_beacon, &point, &mut not_available) {
            break;
        }
        
        point.x -= 1;
    }
    println!("Not available: {}", not_available);

}


// 5542900 too high