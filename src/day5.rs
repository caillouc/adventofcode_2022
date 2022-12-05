use std::fs;

fn create_stack(lines: &Vec<&str>) -> Vec<Vec<char>> {
    let mut stack: Vec<Vec<char>> = vec![vec![]; 9];
    for l in lines {
        if *l == "" {
            break;
        }
        for i in 0..9 {
            let upper = if i == 9 {(i+1) * 4} else { (i+1)*4 - 1};
            let sub = &l[i*4..upper];
            if !sub.contains("[") {
               continue; 
            } else {
                let c: char = sub.chars().nth(1).unwrap();
                if c.is_numeric() {
                    break;
                }
                stack[i].push(c);
            }
        }
    }
    stack.iter_mut().for_each(|s| s.reverse());
    return stack;
}

pub fn res() -> String {
    let file_path = "input/day5";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let lines: Vec<&str> = lines[0..lines.len() - 1].to_vec();

    let mut stack: Vec<Vec<char>> = create_stack(&lines);

    let (_, moves) = lines.split_at(lines.iter().position(|r| *r == "").unwrap());
    for m in moves[1..].iter() {
        let splitted: Vec<&str> = m.split(' ').collect();
        let source = splitted[3].parse::<usize>().unwrap();
        let destination = splitted[5].parse::<usize>().unwrap();
        let mut to_move: Vec<char> = Vec::new();

        for _ in 0..splitted[1].parse::<i32>().unwrap() {
            to_move.push(stack[source - 1].pop().unwrap());
        }
        // Part 2
        to_move.reverse();

        stack[destination - 1].append(&mut to_move);
    }
    
    let mut ret: String = String::new();
    for i in 0..9 {
        ret.push(stack[i].pop().unwrap());
    }

    return ret;
}
