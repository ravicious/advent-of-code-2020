use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("sum of anyone counts: {}", count_anyone_answers(&input));
    println!("sum of everyone counts: {}", count_everyone_answers(&input));
}

fn count_anyone_answers(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|ch| ch.is_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn count_everyone_answers(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut char_to_count = HashMap::new();
            let number_of_lines = group.lines().count();

            group
                .chars()
                .filter(|ch| ch.is_alphabetic())
                .for_each(|ch| {
                    let count = char_to_count.entry(ch).or_insert(0);
                    *count += 1;
                });

            char_to_count
                .iter()
                .filter(|(_, &count)| count == number_of_lines)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc\n\
\n\
a\n\
b\n\
c\n\
\n\
ab\n\
ac\n\
\n\
a\n\
a\n\
a\n\
a\n\
\n\
b";

    #[test]
    fn correctly_counts_anyone_answers() {
        assert_eq!(11, count_anyone_answers(INPUT));
    }

    #[test]
    fn correctly_counts_everyone_answers() {
        assert_eq!(6, count_everyone_answers(INPUT));
    }
}
