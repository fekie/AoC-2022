// https://adventofcode.com/2022/day/6

use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");
const PART_1_MARKER_SIZE: usize = 4;
const PART_2_MARKER_SIZE: usize = 14;

fn main() {
    let chars = INPUT.chars().collect::<Vec<char>>();

    // Part 1
    let first_marker = first_marker_char_number(&chars, PART_1_MARKER_SIZE);

    println!(
        "First Marker (size {}) Completed At Character Number: {}",
        PART_1_MARKER_SIZE, first_marker
    );

    // Part 1
    let first_marker = first_marker_char_number(&chars, PART_2_MARKER_SIZE);

    println!(
        "First Marker (size {}) Completed At Character Number: {}",
        PART_2_MARKER_SIZE, first_marker
    );
}

/// Returns the number of the last character that completes the marker.
/// Note that this does not return the index, but the character number.
fn first_marker_char_number(chars: &[char], marker_len: usize) -> u64 {
    for i in 0..chars.len() {
        let slice = &chars[i..i + marker_len];
        if all_unique(slice) {
            return (i + marker_len) as u64;
        }
    }

    panic!("No marker found!")
}

fn all_unique(chars: &[char]) -> bool {
    let mut seen = HashSet::new();

    for char in chars {
        seen.insert(char);
    }

    seen.len() == chars.len()
}
