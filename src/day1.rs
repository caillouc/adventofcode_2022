use std::fs;

pub fn res() -> i32 {
    let file_path = "input/day1";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    fn rec(max: Vec<i32>, acc: i32, lines: Vec<&str>) -> Vec<i32> {
        if lines.len() == 0 {
            return max;
        }
        let (head, tail) = lines.split_at(1);
        if head[0].eq("") {
            let min = max.iter().min().unwrap();
            // println!("maxes : {:?}", max);
            // println!("min : {}", *min);
            if acc > *min {
                let new_max: Vec<i32> = max
                    .iter()
                    .map(|x| if x == min { acc } else { *x })
                    .collect();
                return rec(new_max, 0, tail.to_vec());
            }
            return rec(max, 0, tail.to_vec());
        } else {
            return rec(max, acc + head[0].parse::<i32>().unwrap(), tail.to_vec());
        }
    }
    let maxes: Vec<i32> = rec(
        vec![0, 1, 2],
        0,
        contents.split('\n').collect::<Vec<&str>>(),
    );
    // println!("{}", maxes.iter().sum::<i32>());
    return maxes.iter().sum::<i32>();
}
