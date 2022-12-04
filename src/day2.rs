use std::collections::HashMap;
use std::fs;

pub fn res() -> i32 {
    let file_path = "input/day2";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect::<Vec<&str>>();

    fn score(op: &str, m: &str) -> i32 {
        let opponent = HashMap::from([
            ("A", 1), // rock
            ("B", 2), // paper
            ("C", 3), // scissors
        ]);

        let _me = HashMap::from([
            ("X", 1), // rock
            ("Y", 2), // paper
            ("Z", 3), // scissors
        ]);

        let me2 = HashMap::from([
            ("X", 1), // lose
            ("Y", 0), // draw
            ("Z", 2), // win
        ]);

        // (opponent - me) mod 3 = 0 -> draw (3)
        // (opponent - me) mod 3 = 2 -> win  (6)
        // (opponent - me) mod 3 = 1 -> lose (0)

        // opponent - me = res
        // me = opponent - res

        // println!("op: {}, m: {}", op, m);

        let points = HashMap::from([
            (0, 3), // draw
            (2, 6), // win
            (1, 0), // lose
        ]);

        // let res: i32 = (opponent.get(op).unwrap() - me.get(m).unwrap() + 3) % 3;
        let res: i32 = *me2.get(m).unwrap();

        // println!("res: {}", res);

        let pts: i32 = *points.get(&res).unwrap();

        // println!("pts: {}", pts);

        // let score: i32 = pts + me.get(m).unwrap();
        let temp = (opponent.get(op).unwrap() - res + 3) % 3;
        let a = if temp == 0 { 3 } else { temp };
        let score: i32 = pts + a;

        // println!("score: {}", score);

        return score;
    }

    fn total(acc: i32, lines: Vec<&str>) -> i32 {
        if lines.len() == 0 {
            return acc;
        }
        let (head, tail) = lines.split_at(1);
        if head[0].eq("") {
            return total(acc, tail.to_vec());
        }
        let opponent = head[0].split(' ').collect::<Vec<&str>>()[0];
        let my_play = head[0].split(' ').collect::<Vec<&str>>()[1];
        let score = score(opponent, my_play);
        return total(acc + score, tail.to_vec());
    }

    // println!("{}", total(0, lines));

    return total(0, lines);
}
