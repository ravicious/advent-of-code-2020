use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!(
        "{} trees in the 3 right 1 down slope",
        count_trees(&input, 3, 1)
    );
    println!(
        "{} trees (multiplied trees from all slopes)",
        count_trees_in_all_slopes(&input)
    );
}

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn count_trees_in_all_slopes(input: &str) -> usize {
    SLOPES
        .iter()
        .map(|(right, down)| count_trees(input, *right, *down))
        .product()
}

fn count_trees(input: &str, right: usize, down: usize) -> usize {
    input
        .lines()
        .step_by(down)
        .enumerate()
        .skip(1)
        .fold(0, |acc, (i, line)| {
            let char_on_position = line
                .chars()
                .nth(right * i % line.len())
                .expect("Attempted to fetch char out of bounds");

            if char_on_position == '#' {
                acc + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......\n\
#...#...#..\n\
.#....#..#.\n\
..#.#...#.#\n\
.#...##..#.\n\
..#.##.....\n\
.#.#.#....#\n\
.#........#\n\
#.##...#...\n\
#...##....#\n\
.#..#...#.#";

    #[test]
    fn handles_example_input_for_first_problem() {
        assert_eq!(7, count_trees(INPUT, 3, 1));
    }

    #[test]
    fn handles_2_down() {
        assert_eq!(2, count_trees(INPUT, 1, 2));
    }

    #[test]
    fn counts_all_slopes() {
        assert_eq!(336, count_trees_in_all_slopes(INPUT));
    }
}
