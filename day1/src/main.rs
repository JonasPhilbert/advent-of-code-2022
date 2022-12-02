fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut elves: Vec<u64> = Vec::new();
    for elf in input.split("\n\n") {
        let mut total_calories: u64 = 0;
        for snack in elf.split("\n") {
            if snack.len() > 0 {
                total_calories += snack.parse::<u64>().unwrap();
            }
        }
        elves.push(total_calories)
    }
    elves.sort_unstable();
    elves.reverse();

    println!("#1: {}", elves[0]);
    println!("#2: {}", elves[0..3].iter().sum::<u64>());
}
