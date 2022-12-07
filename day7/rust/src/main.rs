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
        } else if new_vec[0].chars().nth(0).unwrap().is_ascii_digit() {
            let value = new_vec[0].parse::<usize>().unwrap();
            for i in &path {
                let hmap = library.get(i).unwrap();
                library.insert(i.to_string(), hmap + value);
            }
        }
    }
    let mut total = 0 as usize;
    for (_, value) in &library {
        if value <= &100000 {
            total += value.to_owned()
        }
    }
    let needed_space = 30000000 - (70000000 - library.get("/0").unwrap());
    let mut candidate: Vec<usize> = Vec::new();
    for (_, value) in &library {
        if value >= &needed_space {
            candidate.push(value.to_owned())
        }
    }
    println!("stage 1 = {}", total);
    println!("stage 2 = {}", &candidate.iter().min().unwrap());
}
fn main() {
    stage1()
}
