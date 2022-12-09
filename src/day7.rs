use std::collections::HashMap;
use std::fs;

fn total_path(path: &Vec<&str>, dir_name: &str) -> String { 
    let mut ret = String::new();
    for p in path {
        ret.push_str(p);
        ret.push_str("/");
    }
    ret.push_str(dir_name);
    return ret;
}

pub fn res() -> i32 {
    let file_path = "input/day7";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let lines: Vec<&str> = lines[0..lines.len() - 1].to_vec();

    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut path: Vec<&str> = Vec::new();
    let mut total_size = 0;
    for l in lines {
        // println!("{:?}", path);
        // println!("{:?}", directories);
        // println!("{}", l);
        if l.starts_with('$') {
            let splitted: Vec<&str> = l.split(' ').collect();
            if splitted[1] == "cd" {
                if splitted[2] == ".." {
                    path.pop();
                } else {
                    let dir_path = total_path(&path, splitted[2]);
                    if !directories.contains_key(dir_path.as_str()) {
                        directories.insert(dir_path, 0);
                    }else {
                        println!("Already contains {}", dir_path);
                    }
                    path.push(splitted[2]);
                }
                continue;
            }
            if splitted[1] == "ls" {
                continue;
            }
        } else if l.starts_with("dir") {
        } else {
            let splitted: Vec<&str> = l.split(' ').collect();
            let cur_size = splitted[0].to_string().parse::<i32>().unwrap();
            total_size += cur_size;
            let mut temp_path = String::new();
            for f in &path {
                temp_path.push_str(f);
                let temp = directories.get_mut(temp_path.as_str()).unwrap();
                *temp += cur_size;
                temp_path.push_str("/");
            }
        }
    }

    // Part 1
    // let mut sum: i32 = 0;
    // for (_key, size) in directories {
    //     if size <= 100000 {
    //         sum += size as i32;
    //     }
    // }
    // return sum;

    // Part 2
    let remaining = 70000000 - total_size;
    let need_to_free = 30000000 - remaining;
    let mut ret = total_size;
    for  (_key, size) in directories {
        if size >= need_to_free && size < ret {
            ret = size;
        }
    }
    return ret;
}
