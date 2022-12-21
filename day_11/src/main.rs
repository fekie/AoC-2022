use std::cell::RefCell;

const INPUT: &str = include_str!("../input.txt");

/// If the inner value is `None`, it means to do the operation with itself (old * old).
#[derive(Debug)]
enum Operation {
    Add(Option<u64>),
    Multiply(Option<u64>),
}

#[derive(Debug)]
struct Monkey {
    // We have to use a RefCell here as we need to mutably
    // change values, but not at the same time.
    items: RefCell<Vec<u64>>,
    operation: Operation,
    divisible_by: u64,
    // If the worry value is divisble by self.divisble_by, throw to this monkey.
    on_success: usize,
    // Follows the same logic as self.on_succes
    on_fail: usize,
    total_inspections: RefCell<u64>,
}

fn main() {
    let mut monkeys = generate_monkeys();

    for _ in 0..20 {
        run_cycle(&mut monkeys);
    }

    let monkey_business = calculate_monkey_business(&monkeys);

    println!("Monkey Business: {monkey_business}")
}

fn calculate_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut inspection_totals = monkeys
        .iter()
        .map(|x| *x.total_inspections.borrow())
        .collect::<Vec<u64>>();

    inspection_totals.sort();

    /* dbg!(&inspection_totals); */

    let top = inspection_totals.pop().unwrap();
    let second_top = inspection_totals.pop().unwrap();

    /* dbg!(top, second_top); */

    top * second_top
}

fn run_cycle(monkeys: &mut [Monkey]) {
    for monkey in monkeys.iter() {
        // We drain as we're going to be moving these values to another inventory each time
        for mut item in monkey.items.borrow_mut().drain(..).collect::<Vec<u64>>() {
            // For each item we increment the total inspections
            // Yes I have to assign values like this so they don't conflict
            let new_total_inspections = *monkey.total_inspections.borrow() + 1;
            *monkey.total_inspections.borrow_mut() = new_total_inspections;

            /*  dbg!(&monkey.operation);
            dbg!(&item); */

            // We do the monkey operation
            item = match monkey.operation {
                Operation::Add(possible_n) => match possible_n {
                    Some(n) => item + n,
                    None => item + item,
                },
                Operation::Multiply(possible_n) => match possible_n {
                    Some(n) => item * n,
                    None => item * item,
                },
            };

            /* dbg!(&item); */

            // We do the worry divided by 3 (rounded down)
            item /= 3;

            // We do the test and then throw it if needed
            match (item % monkey.divisible_by) == 0 {
                true => monkeys[monkey.on_success].items.borrow_mut().push(item),
                false => monkeys[monkey.on_fail].items.borrow_mut().push(item),
            }
        }
    }
}

// Just for fun i wanted to cram all the parsing into one iter chain
fn generate_monkeys() -> Vec<Monkey> {
    INPUT
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|monkey_data| {
            let items = RefCell::new(
                monkey_data[1].trim()[("Starting items: ").len()..]
                    .split(", ")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            );

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

            let total_inspections = RefCell::new(0);

            Monkey {
                items,
                operation,
                divisible_by,
                on_success,
                on_fail,
                total_inspections,
            }
        })
        .collect::<Vec<Monkey>>()
}
