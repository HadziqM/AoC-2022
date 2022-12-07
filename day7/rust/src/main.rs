use std::collections::HashMap;
use std::fs;

fn stage1() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut library = HashMap::new();
    let mut path: Vec<String> = Vec::new();
    let mut unique_name = 0 as usize;
    for i in input.lines() {
        let new_vec = i.split(" ").collect::<Vec<_>>();
        if new_vec[0] == "$" {
            if new_vec[1] == "cd" {
                if new_vec[2] == ".." {
                    path.pop();
                } else {
                    let unique_str = format!("{}{}", new_vec[2], unique_name);
                    path.push(unique_str.to_owned());
                    unique_name += 1;
                    library.insert(unique_str.to_owned(), 0 as usize);
                }
            }
        } else if new_vec[0].parse::<usize>().is_ok() {
            let value = new_vec[0].parse::<usize>().unwrap();
            for i in &path {
                let hmap = library.get(i).unwrap();
                library.insert(i.to_string(), hmap + value);
            }
        }
    }

    let total = library
        .values()
        .cloned()
        .filter(|&i| i <= 100000)
        .sum::<usize>();
    let freed_space = library
        .values()
        .cloned()
        .filter(|&i| i >= 30000000 - (70000000 - library.get("/0").unwrap()))
        .min()
        .unwrap();
    println!("stage 1 = {}", total);
    println!("stage 2 = {}", freed_space);
}
fn main() {
    stage1()
}
