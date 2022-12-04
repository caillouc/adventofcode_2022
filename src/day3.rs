use std::fs;

pub fn res() -> i32 {
    let file_path = "input/day3";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect::<Vec<&str>>();

    fn _compute_score1(acc: i32, lines: Vec<&str>) -> i32 {
        if lines.len() == 0 {
            return acc;
        }
        let (head, tail) = lines.split_at(1);
        let comp1 = head[0][0..head[0].len() / 2].to_string();
        let comp2 = head[0][head[0].len() / 2..].to_string();
        for c in comp1.chars() {
            if comp2.contains(c) {
                if c.is_uppercase() {
                    return compute_score1(acc + c as i32 - 64 + 26, tail.to_vec());
                } else {
                    return compute_score1(acc + c as i32 - 96, tail.to_vec());
                }
            }
        }
        return -1;
    }

    fn compute_score2(acc: i32, lines: Vec<&str>) -> i32 {
        if lines.len() == 0 {
            return acc;
        }
        let (head, tail) = lines.split_at(3);
        for c in head[0].chars() {
            if head[1].contains(c) {
                if head[2].contains(c) {
                    if c.is_uppercase() {
                        return compute_score2(acc + c as i32 - 64 + 26, tail.to_vec());
                    } else {
                        return compute_score2(acc + c as i32 - 96, tail.to_vec());
                    }
                }
            }
        }
        return -1;
    }

    // println!("{}", compute_score2(0, lines[0..lines.len()-1].to_vec()));

    return compute_score2(0, lines[0..lines.len() - 1].to_vec());
}
