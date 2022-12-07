// https://adventofcode.com/2022/day/5

use std::str::Lines;

const INPUT: &str = include_str!("../input.txt");
const STARTING_LINE_OFFSET: usize = 2;

#[derive(Debug)]
struct Dock {
    /// Each stack contains a vector of chars which represent
    /// the creates. The top of the crate stack is represented at
    /// the end of these vectors.
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Instruction {
    amount: u64,
    from_index: usize,
    to_index: usize,
}

fn main() {
    let lines = INPUT.lines();

    // Part 1
    let (stack_amount, stack_details_line_index) = find_stack_numbers_line_details(lines.clone());
    let mut dock = Dock::new(lines.clone(), stack_amount, stack_details_line_index);
    let instructions = generate_instructions(lines.clone(), stack_details_line_index);

    dock.process_instructions(&instructions);
    let top_crates_string = dock.top_crates_string();

    println!("Top Crates Strings: {}", top_crates_string);

    // Part 2
    let (stack_amount, stack_details_line_index) = find_stack_numbers_line_details(lines.clone());
    let mut dock = Dock::new(lines.clone(), stack_amount, stack_details_line_index);
    let instructions = generate_instructions(lines, stack_details_line_index);

    dock.process_instructions_upgraded(&instructions);
    let top_crates_string_upgraded = dock.top_crates_string();

    println!(
        "Top Crates Strings (Upgraded): {}",
        top_crates_string_upgraded
    );
}

/// Returns the amount of stacks, and the index of the line it was found on.
fn find_stack_numbers_line_details(lines: Lines) -> (u64, usize) {
    for (i, line) in lines.into_iter().enumerate() {
        let split = line.split_whitespace();
        let parsed_numbers_count = split.filter(|x| x.parse::<u64>().is_ok()).count() as u64;

        if parsed_numbers_count != 0 {
            return (parsed_numbers_count, i);
        }
    }

    panic!("No parsable line was found!")
}

fn generate_instructions(lines: Lines, stack_details_line_index: usize) -> Vec<Instruction> {
    // We add 1 to the index so it gives us the line number of the details line.
    // We then add another one to skip the blank line.
    lines
        .skip(stack_details_line_index + 1 + 1)
        .map(Instruction::new)
        .collect()
}

impl Dock {
    fn new(lines: Lines, stack_amount: u64, stack_details_line_index: usize) -> Self {
        let stacks = Self::initial_stacks(lines, stack_amount, stack_details_line_index);
        Self { stacks }
    }

    fn process_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            for _ in 0..instruction.amount {
                let c = self.stacks[instruction.from_index].pop().unwrap();
                self.stacks[instruction.to_index].push(c);
            }
        }
    }

    // Retains the crate order when moving.
    fn process_instructions_upgraded(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            let mut crates = Vec::new();

            for _ in 0..instruction.amount {
                let c = self.stacks[instruction.from_index].pop().unwrap();
                crates.push(c);
            }

            crates.reverse();

            for c in crates {
                self.stacks[instruction.to_index].push(c);
            }
        }
    }

    fn top_crates_string(&self) -> String {
        let mut sum_string = String::new();
        for stack in &self.stacks {
            sum_string.push(*stack.last().unwrap());
        }
        sum_string
    }

    fn initial_stacks(
        lines: Lines,
        stack_amount: u64,
        stack_details_line_index: usize,
    ) -> Vec<Vec<char>> {
        // We can use the stack details line index since it is the same as the previous index + 1
        let relevant_lines = lines.take(stack_details_line_index);

        // These crates are horizontal but we need to stack them up.
        //
        let crate_rows: Vec<Vec<Option<char>>> =
            relevant_lines.map(Self::crate_line_to_chars).collect();

        let mut stacks = Vec::new();
        for _ in 0..stack_amount {
            stacks.push(Vec::new());
        }

        for row in crate_rows.iter().rev() {
            for (i, possible_char) in row.iter().enumerate() {
                if let Some(c) = possible_char {
                    stacks[i].push(*c);
                }
            }
        }

        stacks
    }

    fn crate_line_to_chars(crate_line: &str) -> Vec<Option<char>> {
        let raw_line_chars = crate_line.chars().collect::<Vec<char>>();

        // Each crate in the text is 3 characters long, with a space inbetween.
        // We cannot split on the whitespace as not every stack has non-whitespace
        // characters at the top.
        let crate_strings = raw_line_chars
            .chunks(4)
            .map(|x| x[0..3].iter().collect::<String>());

        crate_strings
            .map(|x| Self::crate_string_to_char(&x))
            .collect::<Vec<Option<char>>>()
    }

    fn crate_string_to_char(crate_string: &str) -> Option<char> {
        if crate_string.split_whitespace().count() == 0 {
            return None;
        }

        crate_string.chars().nth(1)
    }
}

impl Instruction {
    fn new(line: &str) -> Self {
        let mut split = line.split_whitespace().skip(1).step_by(2);

        let amount = split.next().unwrap().parse().unwrap();

        // We subtract one from here as we're representing the index, not stack number.
        let from_index = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let to_index = split.next().unwrap().parse::<usize>().unwrap() - 1;

        Self {
            amount,
            from_index,
            to_index,
        }
    }
}
