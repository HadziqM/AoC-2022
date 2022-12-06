use itertools::Itertools;
use std::{collections::VecDeque, fs};

fn stage_1() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut init = VecDeque::new();
    for i in 0..14 {
        init.push_front(input.chars().nth(i))
    }
    let mut i = 0;
    while !init.iter().all_unique() {
        init.pop_back();
        init.push_front(input.chars().nth(i + 13));
        i += 1;
    }
    println!("{}", i + 13);
    println!("{:?}", init);
}
fn main() {
    stage_1()
}
