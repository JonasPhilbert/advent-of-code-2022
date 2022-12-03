fn main() {
    assert_eq!(score(&'a'), 1);
    assert_eq!(score(&'z'), 26);
    assert_eq!(score(&'A'), 27);
    assert_eq!(score(&'Z'), 52);

    let mut sum = 0;
    let input = std::fs::read_to_string("input").unwrap();
    let parsed: Vec<(&str, &str)> = input.split("\n").map(|row| row.split_at(row.len() / 2)).collect();

    for rucksack in parsed {
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

    println!("{}", sum);
}

fn score(char: &char) -> u32 {
    if char.is_lowercase() {
        u32::from(*char) - 96
    } else {
        u32::from(*char) - 38
    }
}
