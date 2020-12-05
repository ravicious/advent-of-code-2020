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

fn binary_space_partitioning(input: &str, mut max: u8, lower_half: char, upper_half: char) -> u8 {
    let mut min = 0;

    for ch in input.chars() {
        let half = (max - min + 1) / 2;

        if ch == lower_half {
            max -= half;
        } else if ch == upper_half {
            min += half;
        } else {
            panic!(
                "Unknown character {}, expected {} or {}",
                ch, lower_half, upper_half
            )
        }
    }

    min
}

fn calculate_row(input: &str) -> u8 {
    binary_space_partitioning(input, 127, 'F', 'B')
}

fn calculate_column(input: &str) -> u8 {
    binary_space_partitioning(input, 7, 'L', 'R')
}

fn calculate_seat_id(input: &str) -> u16 {
    let (row_input, column_input) = input.split_at(7);
    let row = calculate_row(row_input);
    let column = calculate_column(column_input);

    (row as u16) * 8 + (column as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_row() {
        assert_eq!(44, calculate_row("FBFBBFF"));
        assert_eq!(70, calculate_row("BFFFBBF"));
        assert_eq!(14, calculate_row("FFFBBBF"));
        assert_eq!(102, calculate_row("BBFFBBF"));
    }

    #[test]
    fn calculates_column() {
        assert_eq!(5, calculate_column("RLR"));
        assert_eq!(7, calculate_column("RRR"));
        assert_eq!(4, calculate_column("RLL"));
    }

    #[test]
    fn calculates_seat_id() {
        assert_eq!(567, calculate_seat_id("BFFFBBFRRR"));
        assert_eq!(119, calculate_seat_id("FFFBBBFRRR"));
        assert_eq!(820, calculate_seat_id("BBFFBBFRLL"));
    }
}
