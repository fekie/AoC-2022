// https://adventofcode.com/2022/day/8

const INPUT: &str = include_str!("../input.txt");

struct Trees {
    pub grid: Vec<Vec<u8>>,
}

fn main() {
    // The grid size is (99x99).
    // This means the max index for x and y is 98.
    let grid = INPUT
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap() as u8).collect())
        .collect::<Vec<Vec<u8>>>();

    let trees = Trees::new(grid);

    // Part 1
    let mut visible_amount = 0;

    for x in 0..trees.grid[0].len() {
        for y in 0..trees.grid.len() {
            if trees.is_visible(x, y) {
                visible_amount += 1;
            }
        }
    }

    println!("Amount Of Visible Trees: {}", visible_amount);

    // Part 2
    let mut highest_scenic_score = 0;

    for x in 0..trees.grid[0].len() {
        for y in 0..trees.grid.len() {
            let score = trees.scenic_score(x, y);
            if highest_scenic_score < score {
                highest_scenic_score = score
            }
        }
    }

    println!("High Scenic Score: {}", highest_scenic_score);
}

impl Trees {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        Self { grid }
    }

    /// We use this method for indexing so we can
    /// have the origin at the bottom left.
    fn index(&self, x: usize, y: usize) -> u8 {
        let max_y_index = self.grid.len() - 1;

        self.grid[max_y_index - y][x]
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.index(x, y);

        let above = self.above(x, y);
        let right = self.right(x, y);
        let below = self.below(x, y);
        let left = self.left(x, y);

        if above.is_empty() || right.is_empty() || below.is_empty() || left.is_empty() {
            return true;
        }

        Self::is_visible_behind_given_trees(tree, &above)
            || Self::is_visible_behind_given_trees(tree, &right)
            || Self::is_visible_behind_given_trees(tree, &below)
            || Self::is_visible_behind_given_trees(tree, &left)
    }

    fn is_visible_behind_given_trees(chosen_tree: u8, trees: &[u8]) -> bool {
        for tree in trees {
            if chosen_tree <= *tree {
                return false;
            }
        }

        true
    }

    fn scenic_score(&self, x: usize, y: usize) -> u64 {
        let tree = self.index(x, y);

        let above = self.above(x, y);
        let right = self.right(x, y);
        let below = self.below(x, y);
        let left = self.left(x, y);

        Self::viewable_tree_amount(tree, &above)
            * Self::viewable_tree_amount(tree, &right)
            * Self::viewable_tree_amount(tree, &below)
            * Self::viewable_tree_amount(tree, &left)
    }

    fn viewable_tree_amount(chosen_tree: u8, trees: &[u8]) -> u64 {
        let mut viewable = 0;

        for tree in trees {
            viewable += 1;

            if chosen_tree <= *tree {
                break;
            }
        }

        viewable
    }

    fn above(&self, x: usize, y: usize) -> Vec<u8> {
        let mut trees = Vec::new();
        let max_y_index = self.grid.len() - 1;

        if y >= max_y_index {
            return trees;
        }

        let start_y = y + 1;
        let end_y = max_y_index;

        for y in start_y..=end_y {
            trees.push(self.index(x, y));
        }

        trees
    }

    fn below(&self, x: usize, y: usize) -> Vec<u8> {
        let mut trees = Vec::new();

        if y == 0 {
            return trees;
        }

        let start_y = y - 1;
        let end_y = 0;

        for y in (end_y..=start_y).rev() {
            trees.push(self.index(x, y));
        }

        trees
    }

    fn right(&self, x: usize, y: usize) -> Vec<u8> {
        let mut trees = Vec::new();
        let max_x_index = self.grid[0].len() - 1;

        if x >= max_x_index {
            return trees;
        }

        let start_x = x + 1;
        let end_x = max_x_index;

        for x in start_x..=end_x {
            trees.push(self.index(x, y));
        }

        trees
    }

    fn left(&self, x: usize, y: usize) -> Vec<u8> {
        let mut trees = Vec::new();

        if x == 0 {
            return trees;
        }

        let start_x = x - 1;
        let end_x = 0;

        for x in (end_x..=start_x).rev() {
            trees.push(self.index(x, y));
        }

        trees
    }
}
