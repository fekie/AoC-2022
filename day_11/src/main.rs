const INPUT: &str = include_str!("../input.txt");

/// If the inner value is `None`, it means to do the operation with itself (old * old).
#[derive(Debug)]
enum Operation {
    Add(Option<u64>),
    Multiply(Option<u64>),
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    // If the worry value is divisble by self.divisble_by, throw to this monkey.
    on_success: usize,
    // Follows the same logic as self.on_succes
    on_fail: usize,
}

fn main() {
    let monkeys = generate_monkeys();
    dbg!(monkeys);
}

// Just for fun i wanted to cram all the parsing into one iter chain
fn generate_monkeys() -> Vec<Monkey> {
    INPUT
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .enumerate()
        .map(|(id, monkey_data)| {
            let mut items = monkey_data[1].trim()[("Starting items: ").len()..]
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            items.sort();

            let operation = {
                let operation_data = monkey_data[2].trim()["Operation: new = old ".len()..]
                    .split_whitespace()
                    .collect::<Vec<&str>>();

                match operation_data[0] {
                    "+" => Operation::Add(operation_data[1].parse().ok()),
                    "*" => Operation::Multiply(operation_data[1].parse().ok()),
                    _ => panic!("Could not parse operation data"),
                }
            };

            let divisible_by = monkey_data[3]
                .trim()
                .chars()
                .nth("Test: divisible by ".len())
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let on_success = monkey_data[4]
                .trim()
                .chars()
                .nth("If true: throw to monkey ".len())
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();

            let on_fail = monkey_data[5]
                .trim()
                .chars()
                .nth("If false: throw to monkey ".len())
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();

            Monkey {
                id,
                items,
                operation,
                divisible_by,
                on_success,
                on_fail,
            }
        })
        .collect::<Vec<Monkey>>()
}
