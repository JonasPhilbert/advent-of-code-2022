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
    // let comma_sep: Vec<String> = elves.iter().map(String::from).collect();
    elves.sort_unstable();
    elves.reverse();
    do_print(elves);
}

fn do_print<T: ToString>(collection: Vec<T>) {
    let strs = collection.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ");
    println!("{}", strs);
}
