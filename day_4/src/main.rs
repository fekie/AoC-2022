// https://adventofcode.com/2022/day/4/

use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

struct AssignmentPair {
    first_elf: RangeInclusive<u64>,
    second_elf: RangeInclusive<u64>,
}

fn main() {
    let all_assignments = generate_assignment_pairs();

    // Part 1
    let fully_overlapping_pairs_amount = amount_of_fully_overlapping_pairs(&all_assignments);
    println!(
        "Amount Of Fully Overlapping Pairs: {}",
        fully_overlapping_pairs_amount
    );

    // Part 2
    let partially_overlapping = amount_of_partially_overlapping_pairs(&all_assignments);
    println!(
        "Amount Of Partially Overlapping Pairs: {}",
        partially_overlapping
    );
}

fn generate_assignment_pairs() -> Vec<AssignmentPair> {
    INPUT.lines().map(AssignmentPair::from_string).collect()
}

fn amount_of_fully_overlapping_pairs(all_assignments: &[AssignmentPair]) -> u64 {
    all_assignments
        .iter()
        .filter(|x| x.fully_overlaps())
        .count() as u64
}

fn amount_of_partially_overlapping_pairs(all_assignments: &[AssignmentPair]) -> u64 {
    all_assignments
        .iter()
        .filter(|x| x.partially_overlaps())
        .count() as u64
}

impl AssignmentPair {
    fn from_string(input: &str) -> Self {
        let mut split = input.split(',');
        let first_elf = Self::raw_assignment_to_range(split.next().unwrap());
        let second_elf = Self::raw_assignment_to_range(split.next().unwrap());

        Self {
            first_elf,
            second_elf,
        }
    }

    fn fully_overlaps(&self) -> bool {
        if self.first_elf.contains(self.second_elf.start())
            && self.first_elf.contains(self.second_elf.end())
        {
            return true;
        }

        if self.second_elf.contains(self.first_elf.start())
            && self.second_elf.contains(self.first_elf.end())
        {
            return true;
        }

        false
    }

    fn partially_overlaps(&self) -> bool {
        if self.first_elf.contains(self.second_elf.start())
            || self.first_elf.contains(self.second_elf.end())
        {
            return true;
        }

        if self.second_elf.contains(self.first_elf.start())
            || self.second_elf.contains(self.first_elf.end())
        {
            return true;
        }

        false
    }

    fn raw_assignment_to_range(input: &str) -> RangeInclusive<u64> {
        let mut split = input.split('-');
        let start = split.next().unwrap().parse().unwrap();
        let end = split.next().unwrap().parse().unwrap();

        start..=end
    }
}
