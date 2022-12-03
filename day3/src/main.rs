fn main() {
    assert_eq!(score(&'a'), 1);
    assert_eq!(score(&'z'), 26);
    assert_eq!(score(&'A'), 27);
    assert_eq!(score(&'Z'), 52);

    part_one();
    part_two();
}

fn part_one() {
    let mut sum = 0;
    let input = std::fs::read_to_string("input").unwrap();
    let rucksacks: Vec<(&str, &str)> = input.split("\n").map(|row| row.split_at(row.len() / 2)).collect();

    for rucksack in rucksacks {
        assert_eq!(rucksack.0.len(), rucksack.1.len());
        for item_a in rucksack.0.chars() {
            let mut found = false;
            for item_b in rucksack.1.chars() {
                if item_a == item_b {
                    sum += score(&item_a);
                    found = true;
                    break;
                }
            }
            if found { break }
        }
    }

    println!("#1: {}", sum);
}

fn part_two() {
    let mut sum = 0;

    let input = std::fs::read_to_string("input").unwrap();
    let rucksacks: Vec<&str> = input.split("\n").collect();
    for i in (0..rucksacks.len() - 3).step_by(3) {
        let a = rucksacks[i];
        let b = rucksacks[i + 1];
        let c = rucksacks[i + 2];
        for char in a.chars() {
            if b.contains(char) && c.contains(char) {
                sum += score(&char);
                break;
            }
        }
    }

    println!("#2: {}", sum);
}

fn score(char: &char) -> u32 {
    if char.is_lowercase() {
        u32::from(*char) - 96
    } else {
        u32::from(*char) - 38
    }
}
