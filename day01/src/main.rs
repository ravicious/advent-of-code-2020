use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let first_result =
        fix_expense_report(2, &input).expect("Couldn't find two values which sum up to 2020");
    let second_result =
        fix_expense_report(3, &input).expect("Couldn't find three values which sum up to 2020");
    println!("{}, {}", first_result, second_result);
}

fn fix_expense_report(combination_size: usize, input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .combinations(combination_size)
        .find(|combination| combination.iter().sum::<u32>() == 2020)
        .map(|combination| combination.iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_first_example_input() {
        let input = [1721, 979, 366, 299, 675, 1456]
            .iter()
            .map(|i| i.to_string())
            .join("\n");
        assert_eq!(Some(514579), fix_expense_report(2, &input));
    }

    #[test]
    fn handles_second_example_input() {
        let input = [1721, 979, 366, 299, 675, 1456]
            .iter()
            .map(|i| i.to_string())
            .join("\n");
        assert_eq!(Some(241861950), fix_expense_report(3, &input));
    }
}
