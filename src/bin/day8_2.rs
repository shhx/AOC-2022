
fn get_pos_blocker(split: Vec<u32>, value: &u32) -> (usize, u32) {
    let len = split.len();
    for (i, v) in split.iter().enumerate() {
        if v >= value {
            return (i+1, *v);
        }
    }
    (len, 0)
}

fn get_score(fil: &Vec<u32>, split: usize, value: &u32) -> usize {
    let (fhalf, shalf) = fil.split_at(split);
    let (mut f_pos_max, max_f) = get_pos_blocker(fhalf.iter().rev().cloned().collect::<Vec<u32>>(), value);
    let (mut s_pos_max, max_s) = get_pos_blocker(shalf.iter().skip(1).cloned().collect::<Vec<u32>>(), value);
    // println!("Split at {}: {:?} val: {value}", split, (fhalf, shalf));
    // println!("Blocker at {:?}: {} -> {}", fhalf, f_pos_max, max_f);
    // println!("Blocker at {:?}: {} -> {}", shalf.iter().skip(1).cloned().collect::<Vec<u32>>(), s_pos_max, max_s);
    let mut score_left = f_pos_max;
    let mut score_right = s_pos_max;
    if fhalf.len() == 0 {
        score_left = 0;
    }
    if shalf.len() == 0 {
        score_right = 0;
    }
    let score = score_left * score_right;
    // println!("scores: L:{} R:{} -> {score}", score_left, score_right);
    score
}

fn main() {
    let input = include_str!("day8_input.txt")
                .lines()
                .map(|l| l.chars()
                .map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
                .collect::<Vec<_>>();
    let n = input.len();
    let m = input[0].len();
    println!("n: {}, m: {}", n, m);
    let mut max_score = 0;
    for i in 0..n {
        for j in 0..m {
            // if i < j {
            //     continue;
            // }
            let row = &input[i];
            let mut score = get_score(row, j, &input[i][j]);
            // *value &= (input[i][j] <= *max_fsplit && input[i][j] <= *max_ssplit) as u32;
            let col = &input.iter().map(|r| r[j]).collect::<Vec<_>>();
            score *= get_score(col, i, &input[i][j]);
            max_score = max_score.max(score);
            // println!("score ({i},{j}) -> {score}");
            // println!("");
        }        
    }
    println!("score: {}", max_score);
}
