use std::fs;

enum Game {
    Rock,
    Paper,
    Scissor,
}
enum Player {
    Opponent,
    Hero,
}
//Stage 2
enum Ending {
    Win,
    Lose,
    Draw,
}
//Stage 1
fn parsed(data: &str) -> Option<Game> {
    match data {
        "A" => Some(Game::Rock),
        "B" => Some(Game::Paper),
        "C" => Some(Game::Scissor),
        "X" => Some(Game::Rock),
        "Y" => Some(Game::Paper),
        "Z" => Some(Game::Scissor),
        _ => None,
    }
}
//Stage 2
fn second_parse(data: &str) -> Option<Ending> {
    match data {
        "X" => Some(Ending::Lose),
        "Y" => Some(Ending::Draw),
        "Z" => Some(Ending::Win),
        _ => None,
    }
}
impl crate::Game {
    fn points(&self, opp: &Game) -> u32 {
        match self {
            Game::Rock => match opp {
                Game::Rock => 4,
                Game::Paper => 1,
                Game::Scissor => 7,
            },
            Game::Paper => match opp {
                Game::Rock => 8,
                Game::Paper => 5,
                Game::Scissor => 2,
            },
            Game::Scissor => match opp {
                Game::Rock => 3,
                Game::Paper => 9,
                Game::Scissor => 6,
            },
        }
    }
    //Stage 2
    fn destined(&self, hero: &Ending) -> u32 {
        match self {
            Game::Rock => match hero {
                Ending::Win => 8,
                Ending::Lose => 3,
                Ending::Draw => 4,
            },
            Game::Paper => match hero {
                Ending::Win => 9,
                Ending::Lose => 1,
                Ending::Draw => 5,
            },
            Game::Scissor => match hero {
                Ending::Win => 7,
                Ending::Lose => 2,
                Ending::Draw => 6,
            },
        }
    }
}
fn strategies(player: Player) -> Vec<Game> {
    let input = fs::read_to_string("../input.txt").unwrap();
    let round = input.split("\n");
    let mut out: Vec<Game> = Vec::new();
    for i in round {
        let idk: Vec<&str> = i.split(" ").collect();
        match player {
            Player::Opponent => out.push(parsed(idk[0]).unwrap()),
            Player::Hero => out.push(parsed(idk[1]).unwrap()),
        }
    }
    out
}
//Stage 2
fn second_strategies() -> Vec<Ending> {
    let input = fs::read_to_string("../input.txt").unwrap();
    let round = input.split("\n");
    let mut out: Vec<Ending> = Vec::new();
    for i in round {
        let idk: Vec<&str> = i.split(" ").collect();
        out.push(second_parse(idk[1]).unwrap())
    }
    out
}

fn main() {
    let hero = strategies(Player::Hero);
    let opponent = strategies(Player::Opponent);
    let mut total: u32 = 0;
    for i in 0..hero.len() {
        total += &hero[i].points(&opponent[i])
    }
    println!("Stage 1 = {}", total);
    let second_hero = second_strategies();
    let mut second_total: u32 = 0;
    for i in 0..opponent.len() {
        second_total += &opponent[i].destined(&second_hero[i])
    }
    println!("Stage 2 = {}", second_total)
}
