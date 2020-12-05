use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("{} valid passports", count_valid_passports(&input));
}

const REQUIRED_PASSPORT_FIELDS: [&str; 7] = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

fn count_valid_passports(input: &str) -> usize {
    input.split("\n\n").fold(0, |acc, lines| {
        let mut passport_fields: Vec<&str> = lines
            .split_whitespace()
            .map(|field_with_value| field_with_value.split(':').next().unwrap())
            .filter(|&field| field != "cid")
            .collect();

        passport_fields.sort_unstable();

        if passport_fields == REQUIRED_PASSPORT_FIELDS {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
byr:1937 iyr:2017 cid:147 hgt:183cm\n\
\n\
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
hcl:#cfa07d byr:1929\n\
\n\
hcl:#ae17e1 iyr:2013\n\
eyr:2024\n\
ecl:brn pid:760753108 byr:1931\n\
hgt:179cm\n\
\n\
hcl:#cfa07d eyr:2025 pid:166559648\n\
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn handles_example_input_for_first_problem() {
        assert_eq!(2, count_valid_passports(INPUT));
    }
}
