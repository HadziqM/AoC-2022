use std::fs;

fn get_calories_vect() -> Vec<i32> {
    let input = fs::read_to_string("../input.txt").unwrap();
    let elves = input.split("\n\n");
    let mut ration: Vec<i32> = Vec::new();
    for iter in elves {
        let mut total: i32 = 0;
        for second in iter.split("\n") {
            total += second.parse::<i32>().unwrap_or(0);
        }
        ration.push(total);
    }
    ration
}
fn get_3_top(mut input: Vec<i32>) -> u32 {
    let mut i: u8 = 0;
    let mut total: u32 = 0;
    while i < 3 {
        let highest = &input.iter().max().unwrap().to_owned();
        input.retain(|&x| x != highest.to_owned());
        i += 1;
        total += highest.to_owned() as u32
    }
    total
}
fn main() {
    let calories = get_calories_vect();
    println!("Stage 1 = {}", &calories.iter().max().unwrap());
    println!("Stage 2 = {}", get_3_top(calories.to_owned()))
}
