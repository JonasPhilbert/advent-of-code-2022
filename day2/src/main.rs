use std::fmt::Display;

enum Choice {
    Rock,
    Paper,
    Scissor,
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
    fn parse(player: &str, opponent: &str) -> Self {
        Self {
            player: match player {
                "X" => Choice::Rock,
                "Y" => Choice::Paper,
                "Z" => Choice::Scissor,
                _ => panic!("Unknown player choice {}!", player),
            },
            opponent: match opponent {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Scissor,
                _ => panic!("Known opponent choice {}!", opponent),
            }
        }
    }

    fn score(&self) -> u16 {
        let choice = match self.player {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        };

        let outcome = match self.player {
            Choice::Rock => match self.opponent {
                Choice::Rock => 3,
                Choice::Paper => 0,
                Choice::Scissor => 6,
            },
            Choice::Paper => match self.opponent {
                Choice::Rock => 6,
                Choice::Paper => 3,
                Choice::Scissor => 0,
            }
            Choice::Scissor => match self.opponent {
                Choice::Rock => 0,
                Choice::Paper => 6,
                Choice::Scissor => 3,
            }
        };

        choice + outcome
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (P) vs. {} (O) => {}", self.player, self.opponent, self.score())
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut rounds: Vec<Round> = Vec::new();
    for play in input.split("\n") {
        if play.len() > 0 {
            let mut iter = play.split(" ");
            let opponent = iter.next().unwrap();
            let player = iter.next().unwrap();
            rounds.push(Round::parse(player, opponent));
            // println!("{} vs {}", opponent, player);
        }
    }

    for round in rounds.iter() {
        println!("{}", round);
    }

    println!();

    let sum = rounds.iter().map(Round::score).sum::<u16>();

    println!("Total: {}", sum);
}
