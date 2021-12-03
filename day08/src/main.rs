use linked_hash_set::LinkedHashSet;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!(
        "Value of acc before loop: {}",
        accumulator_value_before_loop(&input)
    );
    println!(
        "Value of acc after fix: {}",
        accumulator_value_after_fix(&input)
    );
}

fn accumulator_value_before_loop(input: &str) -> i32 {
    let instructions: Vec<_> = input.lines().collect();
    let mut acc: i32 = 0;
    let mut index = 0;
    let mut visited_indices = HashSet::new();

    while let Some(instruction) = instructions.get(index) {
        if visited_indices.contains(&index) {
            break;
        }
        visited_indices.insert(index);

        let (operation, raw_argument) = instruction.split_at(3);
        let argument: i32 = raw_argument[1..].parse().unwrap_or_else(|_| {
            panic!(
                "Invalid argument ({}) for operation {}",
                raw_argument, operation
            )
        });

        match operation {
            "nop" => {
                index += 1;
            }
            "acc" => {
                acc += argument;
                index += 1;
            }
            "jmp" => {
                index = ((index as i32) + argument).try_into().unwrap();
            }
            _ => panic!("Unknown operation {}", operation),
        }
    }

    acc
}

// TODO: Upon encountering nop or jmp, check if toggling it causes the program to terminate. If so,
// return the acc value, otherwise continue to the next instruction.
//
// Write a recursive function there.
fn accumulator_value_after_fix(input: &str) -> i32 {
    let instructions: Vec<_> = input.lines().collect();
    let mut acc: i32 = 0;
    let mut index = 0;
    let mut problematic_index: Option<usize> = None;
    let mut visited_indices = LinkedHashSet::new();

    while let Some(instruction) = instructions.get(index) {
        println!("{}, {}", index, instruction);
        if visited_indices.contains(&index) && Some(index) != problematic_index {
            // Attempt to fix the instruction list.
            if problematic_index.is_none() {
                problematic_index = visited_indices.pop_back();
                index = dbg!(problematic_index.unwrap());
                visited_indices.clear();
                continue;
            } else {
                panic!("We ran into a loop")
            }
        }
        visited_indices.insert(index);

        let (mut operation, raw_argument) = instruction.split_at(3);
        let argument: i32 = raw_argument[1..].parse().unwrap_or_else(|_| {
            panic!(
                "Invalid argument ({}) for operation {}",
                raw_argument, operation
            )
        });

        if Some(index) == problematic_index {
            dbg!(operation);
            match operation {
                "nop" => operation = "jmp",
                "jmp" => operation = "nop",
                _ => {}
            }
        }

        match operation {
            "nop" => {
                index += 1;
            }
            "acc" => {
                acc += argument;
                index += 1;
            }
            "jmp" => {
                index = ((index as i32) + argument).try_into().unwrap();
            }
            _ => panic!("Unknown operation {}", operation),
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_accumulator_value_before_loop() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(5, accumulator_value_before_loop(&input));
    }

    #[test]
    fn returns_accumulator_value_after_fix() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(8, accumulator_value_after_fix(&input));
    }
}
