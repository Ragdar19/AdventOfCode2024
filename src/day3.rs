use std::{fs::{self}, str::FromStr};

use regex::Regex;

pub fn call_day3() -> () {
    let text = fs::read_to_string("input_day3.txt").unwrap();
    let do_actions = retrieve_do_actions(&text);
    let mut final_result = 0;
    for action in do_actions {
        final_result += sum_of_multiplications(&action);
    }

    println!("Sum of all multiplications : {final_result}");
}

fn retrieve_do_actions(input: &str) -> Vec<&str> {
    let regex = Regex::new(r"(?ms)(.*?)(don't\(\)|do\(\))(.*)$").unwrap();

    let mut result = vec![];
    let mut remaining_text = input;
    let mut previous_action = Action::Do;
    while let Some(caps) = regex.captures(remaining_text) {
        let before = caps.get(1).unwrap().as_str();
        let action_word = caps.get(2).unwrap().as_str();
        let after = caps.get(3).unwrap().as_str();

        if previous_action == Action::Do {
            result.push(before);
        }

        if let Ok(action) = Action::from_str(action_word) {
            previous_action = action;
            remaining_text = after;
        }
    }

    
    // last part
    if previous_action == Action::Do {
        result.push(&remaining_text);
    }

    result
}

fn sum_of_multiplications(input: &str) -> i32 {
    // Regex pattern matching mul() with 1-3 digit numbers
    let re = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    let mut result = 0;
    for cap in re.captures_iter(input) {
        let num1 = cap["num1"].parse::<i32>().unwrap();
        let num2 = cap["num2"].parse::<i32>().unwrap();
        result += num1 * num2;
    }

    result
}

#[derive(Debug, PartialEq)]
enum Action {
    Do,
    Dont
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "do()" => Ok(Self::Do),
            "don't()" => Ok(Self::Dont),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_multiplication() {
        let input = "mul(10,20)";
        assert_eq!(sum_of_multiplications(input), 200);
    }

    #[test]
    fn test_multiple_multiplications() {
        let input = "mul(5,6)mul(7,8)mul(9,10)";
        assert_eq!(sum_of_multiplications(input), 30 + 56 + 90);
    }

    #[test]
    fn test_no_multiplications() {
        let input = "some random text";
        assert_eq!(sum_of_multiplications(input), 0);
    }

    #[test]
    fn test_mixed_content() {
        let input = "Start mul(11,12) middle mul(13,14) end";
        assert_eq!(sum_of_multiplications(input), 132 + 182);
    }

    #[test]
    fn test_single_digit_numbers() {
        let input = "mul(1,2)mul(3,4)";
        assert_eq!(sum_of_multiplications(input), 2 + 12);
    }

    #[test]
    fn test_three_digit_numbers() {
        let input = "mul(100,200)mul(999,1)";
        assert_eq!(sum_of_multiplications(input), 20000 + 999);
    }

    #[test]
    fn test_invalid_number_format() {
        let input = "mul(1000,20)"; // 1000 is outside 1-3 digit range
        assert_eq!(sum_of_multiplications(input), 0);
    }

    #[test]
    fn test_retrieve_do_actions_simple() {
        let input = "Hello don't() bye do() success";
        let expected_result = ["Hello ", " success"];
        assert_eq!(retrieve_do_actions(input), expected_result);
    }

    #[test]
    fn test_retrieve_do_actions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_result = ["xmul(2,4)&mul[3,7]!^","?mul(8,5))"];
        assert_eq!(retrieve_do_actions(input), expected_result);
    }

    #[test]
    fn test_retrieve_do_actions_and_multiply() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64]\n(mul(11,8)undo()mul(8,5)do()?mul(8,5))";

        let mut final_result = 0;
        for action in retrieve_do_actions(input) {
            println!("{action}");
            final_result += sum_of_multiplications(&action);
        }
        assert_eq!(final_result, 88);
    }
}
