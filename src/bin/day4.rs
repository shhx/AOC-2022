use std::cmp;

fn is_subset(r1: &Vec<i32>, r2: &Vec<i32>) -> bool {
    if r1[0] <= r2[0] && r1[1] >= r2[1]{
        return true
    }
    return false
}

fn overlaps(r1: &Vec<i32>, r2: &Vec<i32>) -> i32 {
    let min = cmp::max(r1[0], r2[0]);
    let max = cmp::min(r1[1], r2[1]);
    return max - min + 1;
}


fn main() {
    let mut pairs = 0;
    let lines = include_str!("day4_input.txt").lines();
    let assigments = lines.map(|l| l.split(",")).map(|s| s.take(2).collect::<Vec<_>>()).collect::<Vec<_>>();
    // let ranges = assigments.iter()
    //                        .map(|x| x.iter()
    //                        .map(|r| r.split("-")
    //                        .map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>())
    //                        .collect::<Vec<_>>())
    //                        .collect::<Vec<_>>();
    // let lim = ranges.iter().map(|x| x.iter().take(2).inspect(|f| println!("{:?}", f))).collect::<Vec<_>>();
    for a in assigments {
        let r1: Vec<i32> = a[0].split("-").map(|x| x.parse().unwrap()).collect();
        let r2: Vec<i32> = a[1].split("-").map(|x| x.parse().unwrap()).collect();
        // println!("{:?} {:?}", r1, r2);
        if is_subset(&r1, &r2){
            pairs += 1;
        } else if is_subset(&r2, &r1) {
            pairs += 1;
        }
    }
    // .array_chunks(2).map(|x, y| (x, y));
    println!("Pairs {}", pairs);

    // Part 2
    let mut pairs = 0;
    let lines = include_str!("day4_input.txt").lines();
    let assigments = lines.map(|l| l.split(",")).map(|s| s.take(2).collect::<Vec<_>>()).collect::<Vec<_>>();
    for a in assigments {
        let r1: Vec<i32> = a[0].split("-").map(|x| x.parse().unwrap()).collect();
        let r2: Vec<i32> = a[1].split("-").map(|x| x.parse().unwrap()).collect();
        // println!("{:?} {:?}", r1, r2);
        let ov = overlaps(&r1, &r2);
        if ov > 0 {
            // println!("Inside: {}", ov);
            pairs += 1;
        }
    }
    println!("Pairs {}", pairs);

}
