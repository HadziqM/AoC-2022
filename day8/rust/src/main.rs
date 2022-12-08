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
                if up.iter().max().unwrap() < l
                    || bot.iter().max().unwrap() < l
                    || left.iter().max().unwrap() < l
                    || right.iter().max().unwrap() < l
                {
                    total += 1
                }
            }
        }
    }
    println!("stage 1 = {}", total)
}
fn stage2() {}
