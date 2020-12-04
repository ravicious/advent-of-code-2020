use regex::Regex;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let first_count = count_valid_passwords_for_first_problem(&input);
    let second_count = count_valid_passwords_for_second_problem(&input);
    println!("{} valid passwords", first_count);
    println!("{} valid passwords", second_count);
}

fn count_valid_passwords_for_first_problem(input: &str) -> usize {
    let separator = Regex::new(":?[ -]").unwrap();

    input.lines().fold(0, |acc, line| {
        if let [min, max, ch, password] = separator.split(line).collect::<Vec<_>>()[..] {
            let min: usize = min.parse().unwrap();
            let max: usize = max.parse().unwrap();
            let ch_count = password.matches(ch).count();

            if ch_count >= min && ch_count <= max {
                acc + 1
            } else {
                acc
            }
        } else {
            panic!("Invalid line {}", line);
        }
    })
}

fn count_valid_passwords_for_second_problem(input: &str) -> usize {
    let separator = Regex::new(":?[ -]").unwrap();

    input.lines().fold(0, |acc, line| {
        if let [first_position, second_position, ch, password] =
            separator.split(line).collect::<Vec<_>>()[..]
        {
            let first_position = first_position.parse::<usize>().unwrap() - 1;
            let second_position = second_position.parse::<usize>().unwrap() - 1;
            let ch = char::from_str(ch).unwrap();

            if (password.chars().nth(first_position) == Some(ch))
                ^ (password.chars().nth(second_position) == Some(ch))
            {
                acc + 1
            } else {
                acc
            }
        } else {
            panic!("Invalid line {}", line);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_example_input_for_first_problem() {
        let input = "1-3 a: abcde\n\
1-3 b: cdefg\n\
2-9 c: ccccccccc";
        assert_eq!(2, count_valid_passwords_for_first_problem(&input));
    }

    #[test]
    fn handles_example_input_for_second_problem() {
        let input = "1-3 a: abcde\n\
1-3 b: cdefg\n\
2-9 c: ccccccccc";
        assert_eq!(1, count_valid_passwords_for_second_problem(&input));
    }
}
