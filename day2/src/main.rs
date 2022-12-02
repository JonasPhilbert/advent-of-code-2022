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
    fn parse_by_outcome(opponent: &str, outcome: &str) -> Self {
        let opponent = Choice::parse(opponent);
        let player = match outcome {
                "X" => opponent.wins_against(),
                "Y" => opponent.draws_against(),
                "Z" => opponent.looses_against(),
                _ => panic!("Unknown outcome {}!", outcome),
        };
        Self {
            opponent,
            player,
        }
    }

    fn parse_by_player_choice(opponent: &str, player: &str) -> Self {
        Self {
            player: Choice::parse(player),
            opponent: Choice::parse(opponent),
        }
    }

    fn score(&self) -> u16 {
        let choice = match self.player {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        };

        let mut outcome: u16 = 3;
        if self.player.looses_against() == self.opponent {
            outcome = 0;
        } else if self.player.wins_against() == self.opponent {
            outcome = 6;
        }

        choice + outcome
    }
}

impl Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (P) vs. {} (O) => {}", self.player, self.opponent, self.score())
    }
}

fn main() {
    let input = load_file();
    let sum1 = parse1(&input).iter().map(Round::score).sum::<u16>();
    println!("Total of #1: {}", sum1);

    let sum2 = parse2(&input).iter().map(Round::score).sum::<u16>();
    println!("Total of #2: {}", sum2);
}

fn parse1(input: &String) -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();
    for play in input.split("\n") {
        if play.len() > 0 {
            let mut iter = play.split(" ");
            let opponent = iter.next().unwrap();
            let player = iter.next().unwrap();
            rounds.push(Round::parse_by_player_choice(opponent, player));
        }
    }

    return rounds;
}

fn parse2(input: &String) -> Vec<Round> {
    let mut rounds: Vec<Round> = Vec::new();
    for play in input.split("\n") {
        if play.len() > 0 {
            let mut iter = play.split(" ");
            let opponent = iter.next().unwrap();
            let outcome = iter.next().unwrap();
            rounds.push(Round::parse_by_outcome(opponent, outcome));
        }
    }

    return rounds;
}

fn load_file() -> String { std::fs::read_to_string("input").unwrap() }
