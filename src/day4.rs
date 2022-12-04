use std::fs;

struct Section {
    min: i32,
    max: i32,
}

pub fn res() -> i32 {
    let file_path = "input/day4";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let lines: Vec<&str> = lines[0..lines.len() - 1].to_vec();

    fn rec(acc: i32, lines: Vec<&str>) -> i32 {
        if lines.len() == 0 {
            return acc;
        }
        let (head, tail) = lines.split_at(1);

        let section: Vec<&str> = head[0].split(',').collect();
        let temp: Vec<&str> = section[0].split('-').collect();
        let sec1 = Section {
            min: temp[0].parse().unwrap(),
            max: temp[1].parse().unwrap(),
        };
        let temp: Vec<&str> = section[1].split('-').collect();
        let sec2 = Section {
            min: temp[0].parse().unwrap(),
            max: temp[1].parse().unwrap(),
        };
        // Part 1
        // if (sec1.min >= sec2.min && sec1.max <= sec2.max)
        //     || (sec2.min >= sec1.min && sec2.max <= sec1.max)
        // {
        //     return rec(acc + 1, tail.to_vec());
        // }

        // Part 2
        if !(sec1.max < sec2.min || sec1.min > sec2.max) {
            return rec(acc + 1, tail.to_vec());
        }

        return rec(acc, tail.to_vec());
    }

    return rec(0, lines);
}
