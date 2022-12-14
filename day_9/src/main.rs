use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
struct Movement {
    units: usize,
    movement_type: MovementType,
}

#[derive(Debug, Clone, Copy)]
enum MovementType {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
struct World {
    knots: Vec<Position>,

    // Contains all the positions that the tail has visited.
    visited: HashSet<Position>,
}

fn main() {
    let movements = vec![
        Movement {
            units: 5,
            movement_type: MovementType::Up,
        },
        Movement {
            units: 3,
            movement_type: MovementType::Right,
        },
        Movement {
            units: 1,
            movement_type: MovementType::Down,
        },
    ];
    //let movements = generate_movements();
    let mut world = World::new(2);

    world.process_movements(&movements);

    println!("Tail Positions Visited: {}", world.visited_len());
    dbg!(world.knots);
}

fn generate_movements() -> Vec<Movement> {
    INPUT
        .lines()
        .map(|x| {
            let split = x.split_whitespace().collect::<Vec<&str>>();

            let units = split[1].parse().unwrap();

            let movement_type = match split[0] {
                "U" => MovementType::Up,
                "R" => MovementType::Right,
                "D" => MovementType::Down,
                "L" => MovementType::Left,
                _ => panic!("Could not parse character."),
            };

            Movement {
                units,
                movement_type,
            }
        })
        .collect()
}

impl World {
    fn new(knot_amount: u64) -> Self {
        let mut knots = Vec::new();

        for _ in 0..knot_amount {
            knots.push(Position { x: 0, y: 0 })
        }

        Self {
            knots,
            visited: HashSet::new(),
        }
    }

    fn process_movements(&mut self, movements: &[Movement]) {
        for movement in movements {
            self.process_movement(*movement);
        }
    }

    fn process_movement(&mut self, movement: Movement) {
        for _ in 0..movement.units {
            for knot_index in 0..self.knots.len() {
                let position_change = match knot_index {
                    0 => self.movement_type_to_position_change(movement.movement_type),
                    _ => self.position_change_needed(knot_index),
                };

                self.update_knot_position(knot_index, position_change);
                /* if let Some(movement_type) = next_movement_type {
                    self.update_knot_position(knot_index, movement_type);
                } */

                // Find out next
            }

            self.add_tail_position_to_visited();

            /* self.update_head_one_step(movement.movement_type);
            self.update_trailing_knot_one_step(movement.movement_type);
            self.add_tail_position_to_visited(); */
        }
        //panic!("aaa");
    }

    fn position_change_needed(&self, knot_index: usize) -> Position {
        if self.is_knot_touching_owner(knot_index) {
            return Position { x: 0, y: 0 };
        }

        let prev_knot_index = knot_index - 1;

        let x_change = self.knots[prev_knot_index].x - self.knots[knot_index].x;
        let y_change = self.knots[prev_knot_index].y - self.knots[knot_index].y;

        dbg!(y_change);

        assert!(x_change < 3);
        assert!(y_change < 3);

        /* dbg!(self.knots[prev_knot_index]);
        dbg!(self.knots[knot_index]);
        dbg!(x_change, y_change); */

        let mut x_change_needed = match (x_change % 2) == 0 {
            true => x_change / 2,
            false => x_change,
        };

        let mut y_change_needed = match (x_change % 2) == 0 {
            true => y_change / 2,
            false => y_change,
        };

        // We need this hack for literal corner cases.
        if y_change != 0 && y_change.abs() < x_change.abs() {
            dbg!(y_change);
            y_change_needed += y_change
        }

        if x_change != 0 && x_change.abs() < y_change.abs() {
            x_change_needed += x_change
        }

        Position {
            x: x_change_needed,
            y: y_change_needed,
        }
    }

    /// Updates the knot by one step using the [`MovementType`]
    fn movement_type_to_position_change(&mut self, movement_type: MovementType) -> Position {
        match movement_type {
            MovementType::Up => Position { x: 0, y: 1 },
            MovementType::Right => Position { x: 1, y: 0 },
            MovementType::Down => Position { x: 0, y: -1 },
            MovementType::Left => Position { x: -1, y: 0 },
        }
    }

    fn update_knot_position(&mut self, knot_index: usize, position_change: Position) {
        self.knots[knot_index].x += position_change.x;
        self.knots[knot_index].y += position_change.y;
    }

    /* fn update_tail_one_step(&mut self, head_movement_type: MovementType) {
        if !self.is_tail_touching() {
            let new_tail_pos = self.new_tail_position(head_movement_type);

            self.tail.x = new_tail_pos.x;
            self.tail.y = new_tail_pos.y;
        }
    } */

    /// Adds visited tail positions to self.visited
    fn add_tail_position_to_visited(&mut self) {
        self.visited.insert(*self.knots.iter().last().unwrap());
    }

    fn visited_len(&self) -> usize {
        self.visited.len()
    }

    fn is_knot_touching_owner(&self, knot_index: usize) -> bool {
        if knot_index == 0 {
            return false;
        }

        let prev_knot_index = knot_index - 1;

        let x_max = self.knots[prev_knot_index].x + 1;
        let x_min = self.knots[prev_knot_index].x - 1;

        let y_max = self.knots[prev_knot_index].y + 1;
        let y_min = self.knots[prev_knot_index].y - 1;

        (x_min..=x_max).contains(&self.knots[knot_index].x)
            && (y_min..=y_max).contains(&self.knots[knot_index].y)

        /* let x_max = self.head.x + 1;
        let x_min = self.head.x - 1;

        let y_max = self.head.y + 1;
        let y_min = self.head.y - 1;

        (x_min..=x_max).contains(&self.tail.x) && (y_min..=y_max).contains(&self.tail.y) */
    }

    // Assumes the tail is not touching.
    /* fn new_tail_position(&self, head_movement_type: MovementType) -> Position {
        match head_movement_type {
            MovementType::Up => Position {
                x: self.head.x,
                y: self.head.y - 1,
            },
            MovementType::Right => Position {
                x: self.head.x - 1,
                y: self.head.y,
            },
            MovementType::Down => Position {
                x: self.head.x,
                y: self.head.y + 1,
            },
            MovementType::Left => Position {
                x: self.head.x + 1,
                y: self.head.y,
            },
        }
    } */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_position_updating() {
        let mut world = World::new(2);

        let movements = vec![Movement {
            units: 5,
            movement_type: MovementType::Up,
        }];

        world.process_movements(&movements);

        assert_eq!(world.knots[0], Position { x: 0, y: 5 });
        assert_eq!(world.knots[1], Position { x: 0, y: 4 });

        let movements = vec![Movement {
            units: 1,
            movement_type: MovementType::Right,
        }];

        world.process_movements(&movements);

        assert_eq!(world.knots[0], Position { x: 1, y: 5 });
        assert_eq!(world.knots[1], Position { x: 0, y: 4 });

        let movements = vec![Movement {
            units: 1,
            movement_type: MovementType::Right,
        }];

        world.process_movements(&movements);

        assert_eq!(world.knots[0], Position { x: 2, y: 5 });
        assert_eq!(world.knots[1], Position { x: 1, y: 5 });

        let movements = vec![Movement {
            units: 1,
            movement_type: MovementType::Down,
        }];

        world.process_movements(&movements);

        assert_eq!(world.knots[0], Position { x: 2, y: 4 });
        assert_eq!(world.knots[1], Position { x: 1, y: 5 });

        let movements = vec![Movement {
            units: 1,
            movement_type: MovementType::Down,
        }];

        world.process_movements(&movements);

        // trouble is here.
        assert_eq!(world.knots[0], Position { x: 2, y: 3 });
        assert_eq!(world.knots[1], Position { x: 2, y: 4 });

        /* dbg!(world.knots);
        todo!() */
    }
}
