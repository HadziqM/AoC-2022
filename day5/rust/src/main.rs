use std::fs;

fn cheat_in() -> Vec<Vec<char>> {
    let mut out: Vec<_> = Vec::new();
    out.push("FLMW".chars().rev().collect::<Vec<_>>());
    out.push("FMVZB".chars().rev().collect::<Vec<_>>());
    out.push("QLSRVH".chars().rev().collect::<Vec<_>>());
    out.push("JTMPQVSF".chars().rev().collect::<Vec<_>>());
    out.push("WSL".chars().rev().collect::<Vec<_>>());
    out.push("WJRMPVF".chars().rev().collect::<Vec<_>>());
    out.push("FRNPCQJ".chars().rev().collect::<Vec<_>>());
    out.push("BRWZSPHV".chars().rev().collect::<Vec<_>>());
    out.push("WZHGCJMB".chars().rev().collect::<Vec<_>>());
    out
}
fn get_instruction() -> Vec<Vec<usize>> {
    let binding = fs::read_to_string("../input.txt").unwrap();
    let input = binding.lines().collect::<Vec<_>>();
    input[10..]
        .iter()
        .map(|&e| {
            e.split(" ")
                .filter(|&j| j.parse::<usize>().is_ok())
                .map(|k| k.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn first_stage() {
    let mut cheat = cheat_in();
    for i in get_instruction() {
        let mut k = 0;
        while k < i[0] {
            let idk = cheat[i[1] - 1].pop().unwrap();
            cheat[i[2] - 1].push(idk);
            k += 1;
        }
    }
    println!(
        "stage 1 = {}",
        cheat.iter().map(|i| i.last().unwrap()).collect::<String>()
    )
}
fn second_stage() {
    let mut cheat = cheat_in();
    for i in get_instruction() {
        let element = cheat[i[1] - 1].len() - i[0];
        let last_element = &cheat.clone()[i[1] - 1][element..];
        for j in last_element {
            cheat[i[2] - 1].push(j.to_owned());
            cheat[i[1] - 1].pop();
        }
    }
    print!(
        "stage 2 = {}",
        cheat.iter().map(|e| e.last().unwrap()).collect::<String>()
    )
}

fn main() {
    first_stage();
    second_stage()
}
