struct Elf {
    from: u32,
    to: u32,
}

impl Elf {
    fn contains(&self, other: &Elf) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}

struct Pair {
    elf_a: Elf,
    elf_b: Elf,
}

impl Pair {
    fn has_full_containment(&self) -> bool {
        self.elf_a.contains(&self.elf_b) || self.elf_b.contains(&self.elf_a)
    }
}

fn main() {
    let overlap_group_count = std::fs::read_to_string("input").unwrap()
        .split("\n")
        .filter(|row| row.len() > 0)
        .map(|row| {
            println!("{}", row);
            let mut elves = row.split(",").into_iter().map(|elf| {
                let mut ranges = elf.split("-").map(|num| num.parse::<u32>().unwrap());
                Elf {
                    from: ranges.next().unwrap(),
                    to: ranges.next().unwrap(),
                }
            });
            Pair {
                elf_a: elves.next().unwrap(),
                elf_b: elves.next().unwrap(),
            }
        }).filter(|pair| pair.has_full_containment()).count();
    println!("#1: {}", overlap_group_count);
}
