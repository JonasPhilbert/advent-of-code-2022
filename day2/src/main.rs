use std::fmt::Display;

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn wins_against(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissor,
            Choice::Paper => Choice::Rock,
            Choice::Scissor => Choice::Paper,
        }
    }

    fn draws_against(&self) -> Self { 
        match self {
            Choice::Rock => Choice::Rock,
            Choice::Paper => Choice::Paper,
            Choice::Scissor => Choice::Scissor,
        }
    }

    fn looses_against(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissor,
            Choice::Scissor => Self::Rock,
        }
    }

    fn parse(identifier: &str) -> Self {
        match identifier {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("Unknwon choice {}!", identifier),
        }
    }

    fn score_value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Choice::Rock => "Rock",
            Choice::Paper => "Paper",
            Choice::Scissor => "Scissor",
        })
    }
}

struct Round {
    player: Choice,
    opponent: Choice,
}

impl Round {
    fn parse_by_outcome(row: &(String, String)) -> Self {
        let opponent = Choice::parse(row.0.as_str());
        let player = match row.1.as_str() {
                "X" => opponent.wins_against(),
                "Y" => opponent.draws_against(),
                "Z" => opponent.looses_against(),
                _ => panic!("Unknown outcome {}!", row.1.as_str()),
        };
        Self {
            opponent,
            player,
        }
    }

    fn parse_by_player_choice(row: &(String, String)) -> Self {
        Self {
            opponent: Choice::parse(row.0.as_str()),
            player: Choice::parse(row.1.as_str()),
        }
    }

    fn score(&self) -> i32 {
        let mut outcome: i32 = 3;
        if self.player.looses_against() == self.opponent {
            outcome = 0;
        } else if self.player.wins_against() == self.opponent {
            outcome = 6;
        }

        outcome + self.player.score_value()
    }

    fn sum(rounds: Vec<Round>) -> i32 {
        rounds.iter().map(Round::score).sum::<i32>()
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (P) vs. {} (O) => {}", self.player, self.opponent, self.score())
    }
}

fn main() {
    let rows = load_and_parse_file();
    let rounds1: Vec<Round> = rows.iter().map(Round::parse_by_player_choice).collect();
    let rounds2: Vec<Round> = rows.iter().map(Round::parse_by_outcome).collect();

    println!("Total of #1: {}", Round::sum(rounds1));
    println!("Total of #2: {}", Round::sum(rounds2));
}

fn load_and_parse_file() -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    let file = std::fs::read_to_string("input").unwrap();
    for line in file.split("\n") {
        if line.len() > 0 {
            let mut iter = line.split(" ");
            let a = String::from(iter.next().unwrap());
            let b = String::from(iter.next().unwrap());
            result.push((a, b))
        }
    }

    return result;
}
