use regex::RegexBuilder;

#[derive(Debug)]
struct Stack {
    id: u8,
    crates: Vec<char>, // Vec can act as a stack.
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut stacks = vec![
        Stack { id: 1, crates: vec!['W', 'R', 'F'] },
        Stack { id: 2, crates: vec!['T', 'H', 'M', 'C', 'D', 'V', 'W', 'P'] },
        Stack { id: 3, crates: vec!['P', 'M', 'Z', 'N', 'L'] },
        Stack { id: 4, crates: vec!['J', 'C', 'H', 'R'] },
        Stack { id: 5, crates: vec!['C', 'P', 'G', 'H', 'Q', 'T', 'B'] },
        Stack { id: 6, crates: vec!['G', 'C', 'W', 'L', 'F', 'Z'] },
        Stack { id: 7, crates: vec!['W', 'V', 'L', 'Q', 'Z', 'J', 'G', 'C'] },
        Stack { id: 8, crates: vec!['P', 'N', 'R', 'F', 'W', 'T', 'V', 'C', ] },
        Stack { id: 9, crates: vec!['J', 'W', 'H', 'G', 'R', 'S', 'V'] },
    ];

    let input = std::fs::read_to_string("input").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();
    let re = RegexBuilder::new(r"move (\d+) from (\d) to (\d)").multi_line(true).build().unwrap();
    for cap in re.captures_iter(sections[1]) {
        let mut moved = Vec::new();
        for _ in 0..cap[1].parse().unwrap() {
            {
                let from = stacks.iter_mut().find(|e| e.id == cap[2].to_string().parse().unwrap()).unwrap();
                moved.insert(0, from.crates.pop().unwrap());
            }
        }
        let to = stacks.iter_mut().find(|e| e.id == cap[3].to_string().parse().unwrap()).unwrap();
        moved.iter().for_each(|mov| to.crates.push(*mov));
    }

    let result: Vec<&char> = stacks.iter().map(|stack| stack.crates.last().unwrap()).collect();
    println!("{:?}", result);
}

fn part1() {
    let mut stacks = vec![
        Stack { id: 1, crates: vec!['W', 'R', 'F'] },
        Stack { id: 2, crates: vec!['T', 'H', 'M', 'C', 'D', 'V', 'W', 'P'] },
        Stack { id: 3, crates: vec!['P', 'M', 'Z', 'N', 'L'] },
        Stack { id: 4, crates: vec!['J', 'C', 'H', 'R'] },
        Stack { id: 5, crates: vec!['C', 'P', 'G', 'H', 'Q', 'T', 'B'] },
        Stack { id: 6, crates: vec!['G', 'C', 'W', 'L', 'F', 'Z'] },
        Stack { id: 7, crates: vec!['W', 'V', 'L', 'Q', 'Z', 'J', 'G', 'C'] },
        Stack { id: 8, crates: vec!['P', 'N', 'R', 'F', 'W', 'T', 'V', 'C', ] },
        Stack { id: 9, crates: vec!['J', 'W', 'H', 'G', 'R', 'S', 'V'] },
    ];

    let input = std::fs::read_to_string("input").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();
    let re = RegexBuilder::new(r"move (\d+) from (\d) to (\d)").multi_line(true).build().unwrap();
    for cap in re.captures_iter(sections[1]) {
        for _ in 0..cap[1].parse().unwrap() {
            let moved: char;
            {
                let from = stacks.iter_mut().find(|e| e.id == cap[2].to_string().parse().unwrap()).unwrap();
                moved = from.crates.pop().unwrap();
            }
            let to = stacks.iter_mut().find(|e| e.id == cap[3].to_string().parse().unwrap()).unwrap();
            to.crates.push(moved);
        }
    }

    let result: Vec<&char> = stacks.iter().map(|stack| stack.crates.last().unwrap()).collect();
    println!("{:?}", result);
}
