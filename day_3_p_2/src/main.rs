const INPUT: &str = include_str!("../input.txt");
const ASCII_OFFSET: i32 = -96;
const UPPERCASE_OFFSET: i32 = 26;

/// Consists of 3 sacks full of items.
#[derive(Debug)]
struct Group {
    sacks: Vec<Vec<Item>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    inner: char,
}

fn main() {
    let groups = generate_groups();
    let common_items = find_common_items(&groups);
    let total = sum_priorities(&common_items);

    println!("Total Priorities of Groups' Common Items: {}", total);
}

fn generate_groups() -> Vec<Group> {
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let three_line_chunks = lines.chunks(3);

    let mut groups = Vec::new();

    for chunk in three_line_chunks {
        groups.push(Group::from_lines(chunk))
    }

    groups
}

fn find_common_items(groups: &[Group]) -> Vec<Item> {
    let mut common_items = Vec::new();
    for group in groups {
        common_items.push(group.common_item());
    }
    common_items
}

fn sum_priorities(items: &[Item]) -> u64 {
    items.iter().map(|x| x.priority() as u64).sum()
}

impl Group {
    /// Only accepts 3 lines, will panic otherwise.
    fn from_lines(lines: &[&str]) -> Self {
        assert_eq!(lines.len(), 3);

        let mut sacks = vec![Vec::new(), Vec::new(), Vec::new()];

        for (i, line) in lines.iter().enumerate() {
            for char in line.chars() {
                sacks[i].push(Item { inner: char });
            }
        }

        Self { sacks }
    }

    fn common_item(&self) -> Item {
        for item in &self.sacks[0] {
            if self.sacks[1].contains(item) && self.sacks[2].contains(item) {
                return *item;
            }
        }

        panic!("Common item does not exist!")
    }
}

impl Item {
    fn priority(&self) -> u8 {
        match self.inner.is_uppercase() {
            true => {
                (self.inner.to_lowercase().next().unwrap() as i32 + ASCII_OFFSET + UPPERCASE_OFFSET)
                    as u8
            }
            false => (self.inner.to_lowercase().next().unwrap() as i32 + ASCII_OFFSET) as u8,
        }
    }
}
