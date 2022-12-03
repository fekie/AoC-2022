// https://adventofcode.com/2022/day/1

fn main() {
    let parsed_lines = include_str!("../input.txt")
        .lines()
        .map(|chunk| match chunk.parse::<u64>() {
            Ok(x) => Some(x),
            Err(_) => None,
        })
        .collect::<Vec<Option<u64>>>();

    let mut elf_cal_counts = Vec::new();
    let mut needs_regrouping = true;
    for line in parsed_lines {
        if line.is_none() {
            needs_regrouping = true;
            continue;
        }

        if needs_regrouping {
            elf_cal_counts.push(0);
            needs_regrouping = false;
        }

        let i = elf_cal_counts.len() - 1;
        elf_cal_counts[i] += line.unwrap();
    }

    elf_cal_counts.sort();
    elf_cal_counts.reverse();

    println!(
        "Most Calories Held By Elf: {}",
        elf_cal_counts.iter().take(1).sum::<u64>()
    );

    let top_three_total = elf_cal_counts.iter().take(3).sum::<u64>();

    println!("Most Calories Held By Top 3 Elves: {}", top_three_total);
}
