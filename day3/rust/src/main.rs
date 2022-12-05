use std::fs;

fn get_score(char: char) -> i32 {
    let alphabet = (b'A'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();
    let rev: Vec<char> = [&alphabet[26..], &alphabet[..26]].concat();
    rev.iter().position(|&r| r == char).unwrap() as i32 + 1
}

fn file_input() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let rucsack: Vec<&str> = input.split("\n").collect();
    let out = rucsack
        .iter()
        .map(|&e| {
            get_score(
                e[..e.len() / 2]
                    .chars()
                    .filter(|&r| e[e.len() / 2..].contains(r))
                    .collect::<Vec<char>>()[0],
            )
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>();
    println!("stage 1 = {}", out)
}
fn second_stage() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let rucsack = input.split("\n").collect::<Vec<_>>();
    let mut new_vect: Vec<Vec<&str>> = Vec::new();
    for i in 0..rucsack.len() / 3 {
        let mut new_item: Vec<&str> = Vec::new();
        for j in i * 3..i * 3 + 3 {
            new_item.push(&rucsack[j])
        }
        new_vect.push(new_item)
    }
    let out: i32 = new_vect
        .iter()
        .map(|e| {
            get_score(
                e[0].chars()
                    .filter(|&i| e[1].contains(i))
                    .filter(|&j| e[2].contains(j))
                    .collect::<Vec<char>>()[0],
            )
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    println!("stage 2 = {}", out)
}

fn main() {
    file_input();
    second_stage()
}
