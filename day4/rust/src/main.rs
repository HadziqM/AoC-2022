use std::{fs, i32};

fn first_stage() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let out = input
        .lines()
        .map(|e| {
            e.split(",")
                .collect::<Vec<_>>()
                .iter()
                .map(|&i| {
                    i.split("-")
                        .map(|j| j.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|k| (k[0]..k[1] + 1).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .filter(|l| l[0].iter().all(|m| l[1].contains(m)) || l[1].iter().all(|m| l[0].contains(m)))
        .count();
    println!("first stage = {}", out)
}
fn second_stage() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let out = input
        .lines()
        .map(|e| {
            e.split(",")
                .collect::<Vec<_>>()
                .iter()
                .map(|&i| {
                    i.split("-")
                        .map(|j| j.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .map(|k| (k[0]..k[1] + 1).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .filter(|l| {
            for s in &l[0] {
                if l[1].contains(s) {
                    return true;
                }
            }
            return false;
        })
        .count();
    println!("second stage = {}", out)
}

fn main() {
    first_stage();
    second_stage()
}
