use std::fs;

pub fn res() -> i32 {
    // Part 1
    // const BOUND: usize = 4;
    // Part 2
    const BOUND: usize = 14;
    let file_path = "input/day6";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect();
    // let lines: Vec<&str> = lines[0..lines.len() - 1].to_vec();

    for i in 0..lines[0].len() {
        let sub: &str = &lines[0][i..i+BOUND];
        for (j, c) in sub.chars().enumerate() {
            if j == BOUND - 1 {
                return (i + BOUND) as i32;
            }
            if sub[j+1..].contains(c) {
                break;
            }
        }
    }
    return -1;
}
