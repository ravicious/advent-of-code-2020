use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut seat_ids: Vec<_> = input.lines().map(|line| calculate_seat_id(line)).collect();
    seat_ids.sort_unstable();
    let max_seat_id = seat_ids.last().unwrap();

    println!("Max seat ID: {}", max_seat_id);

    let target_seats = seat_ids
        .windows(2)
        .find(|window| window[1] - window[0] == 2)
        .expect("No target seat IDs found");

    println!("Target seats: {:?}", target_seats);
}

fn calculate_seat_id(input: &str) -> u16 {
    let binary: String = input
        .chars()
        .map(|ch| match ch {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!("Unknown character {}", ch),
        })
        .collect();

    u16::from_str_radix(&binary, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_seat_id() {
        assert_eq!(567, calculate_seat_id("BFFFBBFRRR"));
        assert_eq!(119, calculate_seat_id("FFFBBBFRRR"));
        assert_eq!(820, calculate_seat_id("BBFFBBFRLL"));
    }
}
