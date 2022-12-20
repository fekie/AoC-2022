// https://adventofcode.com/2022/day/10

const INPUT: &str = include_str!("../input.txt");

const CRT_WIDTH: u64 = 40;

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn main() {
    let instructions = generate_instructions();

    process_and_display(&instructions);
}

fn process_and_display(instructions: &[Instruction]) {
    // This is the center of the sprite, which is 3x1
    let mut x_pos = 1i64;
    let mut cycle_count = 0u64;

    let mut line_buffer = String::with_capacity(CRT_WIDTH as usize);

    for instruction in instructions {
        let (instruction_cycles, inc) = match instruction {
            Instruction::Addx(inc) => (2, *inc),
            Instruction::Noop => (1, 0),
        };

        for subcycle in 1..=instruction_cycles {
            cycle_count += 1;

            let char = match pixel(cycle_count, x_pos) {
                true => "#",
                false => ".",
            };

            line_buffer.push_str(char);

            if line_buffer.capacity() == line_buffer.len() {
                println!("{line_buffer}");
                line_buffer.clear()
            }

            // If this is the final cycle, increment the register.
            // We do this last.
            if subcycle == instruction_cycles {
                x_pos += inc;
            }
        }
    }
}

// Whether the pixel is on or off.
fn pixel(cycle_count: u64, x_pos: i64) -> bool {
    let pixel = (cycle_count - 1) % (CRT_WIDTH);
    ((pixel as i64 - 1)..=(pixel as i64 + 1)).contains(&x_pos)
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
