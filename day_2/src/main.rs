// https://adventofcode.com/2022/day/2

const RAW_INPUT: &str = include_str!("../input.txt");

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

/// From the user's perspective.
#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

/// Tells the user whether they should cause a win,
/// draw, or loss.
enum Signal {
    Win,
    Draw,
    Loss,
}

fn main() {
    let (enemy_hands, user_hands) = hand_sequences();

    // Part 1
    let hand_score = total_hand_score(&user_hands);
    let outcome_score = total_outcome_score(&enemy_hands, &user_hands);
    let total_score = hand_score + outcome_score;
    println!("Total Score: {}", total_score);

    // Part 2
    let signals = signal_sequence();
    let user_hands = signals_to_hands(&enemy_hands, &signals);
    let hand_score = total_hand_score(&user_hands);
    let outcome_score = total_outcome_score(&enemy_hands, &user_hands);
    let total_score = hand_score + outcome_score;
    println!("Total Score Adjusted For Signals: {}", total_score);
}

/// Converts the raw line strings into two vectors,
/// a sequence of enemy hands and a sequence of your suggested hands.
fn hand_sequences() -> (Vec<Hand>, Vec<Hand>) {
    let lines = RAW_INPUT.lines().filter(|x| !x.is_empty());

    let mut enemy_hands = Vec::new();
    let mut user_hands = Vec::new();

    for line in lines {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let enemy_hand = match split[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!(),
        };
        let user_hand = match split[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!(),
        };

        enemy_hands.push(enemy_hand);
        user_hands.push(user_hand);
    }

    (enemy_hands, user_hands)
}

fn total_hand_score(user_hands: &[Hand]) -> u64 {
    let mut score = 0;
    for hand in user_hands {
        score += hand.score();
    }
    score
}

fn total_outcome_score(enemy_hands: &[Hand], user_hands: &[Hand]) -> u64 {
    let mut score = 0;
    for (enemy_hand, user_hand) in enemy_hands.iter().zip(user_hands) {
        score += Outcome::from_hands(*enemy_hand, *user_hand).score();
    }
    score
}

fn signal_sequence() -> Vec<Signal> {
    let lines = RAW_INPUT.lines().filter(|x| !x.is_empty());

    let mut signals = Vec::new();

    for line in lines {
        let signal = match line.split_whitespace().collect::<Vec<&str>>()[1] {
            "X" => Signal::Loss,
            "Y" => Signal::Draw,
            "Z" => Signal::Win,
            _ => panic!(),
        };

        signals.push(signal);
    }

    signals
}

fn signals_to_hands(enemy_hands: &[Hand], signals: &[Signal]) -> Vec<Hand> {
    let mut user_hands = Vec::new();

    for (enemy_hand, signal) in enemy_hands.iter().zip(signals) {
        let user_hand = match signal {
            Signal::Win => enemy_hand.winning_hand(),
            Signal::Draw => enemy_hand.draw_hand(),
            Signal::Loss => enemy_hand.losing_hand(),
        };
        user_hands.push(user_hand);
    }

    user_hands
}

impl Hand {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn winning_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn draw_hand(&self) -> Hand {
        *self
    }

    fn losing_hand(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

impl Outcome {
    fn from_hands(enemy_hand: Hand, user_hand: Hand) -> Self {
        match enemy_hand {
            Hand::Rock => match user_hand {
                Hand::Rock => Outcome::Draw,
                Hand::Paper => Outcome::Win,
                Hand::Scissors => Outcome::Loss,
            },
            Hand::Paper => match user_hand {
                Hand::Rock => Outcome::Loss,
                Hand::Paper => Outcome::Draw,
                Hand::Scissors => Outcome::Win,
            },
            Hand::Scissors => match user_hand {
                Hand::Rock => Outcome::Win,
                Hand::Paper => Outcome::Loss,
                Hand::Scissors => Outcome::Draw,
            },
        }
    }

    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}
