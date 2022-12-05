// https://adventofcode.com/2022/day/3/

const INPUT: &str = include_str!("../input.txt");
const ASCII_OFFSET: i32 = -96;
const UPPERCASE_OFFSET: i32 = 26;

#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    inner: char,
}

fn main() {
    let rucksacks = generate_rucksacks();
    let common_items = find_common_items(&rucksacks);
    let total = sum_priorities(&common_items);

    println!("Total Sum of Priorities: {}", total);
}

fn generate_rucksacks() -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in INPUT.lines() {
        let rucksack = Rucksack::from_string(line);
        rucksacks.push(rucksack);
    }
    rucksacks
}

fn find_common_items(rucksacks: &[Rucksack]) -> Vec<Item> {
    let mut common_items = Vec::new();
    for rucksack in rucksacks {
        common_items.push(rucksack.common_item());
    }
    common_items
}

fn sum_priorities(items: &[Item]) -> u64 {
    items.iter().map(|x| x.priority() as u64).sum()
}

impl Rucksack {
    fn from_string(string: &str) -> Self {
        let mut first_compartment = Vec::new();
        let mut second_compartment = Vec::new();

        for (i, char) in string.chars().enumerate() {
            match i < (string.len() / 2) {
                true => first_compartment.push(Item { inner: char }),
                false => second_compartment.push(Item { inner: char }),
            }
        }

        Self {
            first_compartment,
            second_compartment,
        }
    }

    fn common_item(&self) -> Item {
        for item in &self.first_compartment {
            if self.second_compartment.contains(item) {
                return *item;
            }
        }

        panic!("Rucksack does not have a common item!")
    }
}

impl Item {
    fn priority(&self) -> u8 {
        match self.inner.is_uppercase() {
            true => {
                (self.inner.to_lowercase().next().unwrap() as i32 + ASCII_OFFSET + UPPERCASE_OFFSET)
                    as u8
            }
            false => (self.inner.to_lowercase().next().unwrap() as i32 + ASCII_OFFSET) as u8,
        }
    }
}
