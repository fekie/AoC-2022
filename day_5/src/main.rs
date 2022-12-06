use std::str::Lines;

const INPUT: &str = include_str!("../input.txt");
const STARTING_LINE_OFFSET: usize = 2;

struct Dock {
    /// Each stack contains a vector of chars which represent
    /// the creates. The top of the crate stack is represented at
    /// the end of these vectors.
    stacks: Vec<Vec<char>>,
}

fn main() {
    let lines = INPUT.lines();
    let dock = Dock::new(lines.clone());
}

impl Dock {
    fn new(lines: Lines) -> Self {
        let (stack_amount, stack_details_line_index) =
            Self::find_stack_numbers_line_details(lines.clone());

        let stacks = Self::initial_stacks(lines, stack_amount, stack_details_line_index);

        Self { stacks }
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
