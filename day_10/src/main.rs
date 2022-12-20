// https://adventofcode.com/2022/day/10

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn main() {
    let instructions = generate_instructions();

    let at_clock_cycles = vec![20, 60, 100, 140, 180, 220];
    let signal_strengths = find_signal_strengths(&instructions, &at_clock_cycles);
    let total = signal_strengths.iter().sum::<i64>();

    println!("Total Strength: {total}");
}

fn find_signal_strengths(instructions: &[Instruction], at_clock_cycles: &[u64]) -> Vec<i64> {
    let mut register = 1i64;
    let mut cycle_count = 0u64;

    let mut strengths = Vec::new();

    for instruction in instructions {
        let (instruction_cycles, inc) = match instruction {
            Instruction::Addx(inc) => (2, *inc),
            Instruction::Noop => (1, 0),
        };

        for subcycle in 1..=instruction_cycles {
            cycle_count += 1;

            if at_clock_cycles.contains(&cycle_count) {
                strengths.push(register * cycle_count as i64);
            }

            // If this is the final cycle, increment the register.
            // We do this last.
            if subcycle == instruction_cycles {
                register += inc;
            }
        }
    }

    assert_eq!(at_clock_cycles.len(), strengths.len());

    strengths
}

fn generate_instructions() -> Vec<Instruction> {
    INPUT
        .lines()
        .map(|x| {
            let split = x.split_whitespace().collect::<Vec<&str>>();
            match split[0] {
                "addx" => Instruction::Addx(split[1].parse().unwrap()),
                "noop" => Instruction::Noop,
                _ => panic!("Could not parse instruction"),
            }
        })
        .collect()
}
