fn main() {
    stage1()
}
fn get_grid() -> Vec<Vec<u32>> {
    let input = std::fs::read_to_string("../input.txt").unwrap();
    input
        .lines()
        .map(|e| {
            e.chars()
                .map(|i| i.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
fn stage1() {
    let mut total = 0 as usize;
    let input = get_grid();
    let mut size: Vec<u32> = Vec::new();
    for (i, j) in input.iter().enumerate() {
        if i == 0 || i == input.len() - 1 {
            total += j.len();
            continue;
        }
        for (k, l) in j.iter().enumerate() {
            if k == 0 || k == j.len() - 1 {
                total += 1
            } else {
                let mut up: Vec<u32> = Vec::new();
                let mut bot: Vec<u32> = Vec::new();
                let mut left: Vec<u32> = Vec::new();
                let mut right: Vec<u32> = Vec::new();
                for o in 0..i {
                    up.push(input[o][k])
                }
                for o in i + 1..input.len() {
                    bot.push(input[o][k])
                }
                for o in 0..k {
                    left.push(input[i][o])
                }
                for o in k + 1..j.len() {
                    right.push(input[i][o])
                }
                let new_up = up.iter().rev().map(|e| e.to_owned()).collect::<Vec<u32>>();
                let new_left = left
                    .iter()
                    .rev()
                    .map(|e| e.to_owned())
                    .collect::<Vec<u32>>();
                if up.iter().max().unwrap() < l
                    || bot.iter().max().unwrap() < l
                    || left.iter().max().unwrap() < l
                    || right.iter().max().unwrap() < l
                {
                    total += 1
                }
                size.push(
                    get_size(&new_up, l)
                        * get_size(&bot, l)
                        * get_size(&new_left, l)
                        * get_size(&right, l),
                );
            }
        }
    }
    println!("stage 1 = {}", total);
    println!("stage 2 = {}", size.iter().max().unwrap())
}
fn get_size(input: &Vec<u32>, val: &u32) -> u32 {
    for (i, j) in input.iter().enumerate() {
        if j >= val {
            return i as u32 + 1;
        }
    }
    return input.len() as u32;
}
