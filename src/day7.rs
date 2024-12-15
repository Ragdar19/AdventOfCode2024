pub fn call_day7() {
    let equations = read_input();
    let total_calibration_result = total_calibration_result(&equations);
    println!("Total calibration result : {total_calibration_result}");
}

fn read_input() -> Vec<Equation> {
    let mut equations = vec![];
    if let Ok(lines) = super::read_lines("input_day7.txt") {
        for line in lines.flatten() {
            let mut first_split = line.split(':');
            let equation_result = first_split.next().unwrap().parse::<u64>().unwrap();
            let equation_values = first_split
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|val_str| val_str.parse::<u64>().unwrap())
                .collect();
            equations.push(Equation {
                result: equation_result,
                values: equation_values,
            });
        }
    }

    equations
}

fn total_calibration_result(equations: &Vec<Equation>) -> u64 {
    let mut result = 0;
    for equation in equations {
        if equation_is_valid(equation) {
            result += equation.result;
        }
    }

    result
}

fn equation_is_valid(equation: &Equation) -> bool {
    let variations = generate_operator_variations(equation.values.len() - 1);
    for variation in variations {
        let result = equation
            .values
            .iter()
            .enumerate()
            .fold(0, |acc, (index, val)| {
                if index == 0 {
                    acc + val
                } else {
                    match variation[index - 1] {
                        Operator::Addition => acc + val,
                        Operator::Multiplication => acc * val,
                        Operator::Concatenation => concat(acc, *val),
                    }
                }
            });
        if result == equation.result {
            return true;
        }
    }

    false
}

fn generate_operator_variations(n: usize) -> Vec<Vec<Operator>> {
    // Base case
    if n == 0 {
        return vec![vec![]];
    }

    // Recursive generation of all possible variations
    let prev_variations = generate_operator_variations(n - 1);

    // Create new variations by adding either Addition or Multiplication or Concatenation
    // to each previous variation
    prev_variations
        .iter()
        .flat_map(|variation| {
            let mut add_variation = variation.clone();
            add_variation.push(Operator::Addition);

            let mut mult_variation = variation.clone();
            mult_variation.push(Operator::Multiplication);

            let mut concat_variation = variation.clone();
            concat_variation.push(Operator::Concatenation);

            vec![add_variation, mult_variation, concat_variation]
        })
        .collect()
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

struct Equation {
    result: u64,
    values: Vec<u64>,
}

#[derive(Clone, PartialEq, Debug)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

fn string_to_equations(text: String) -> Vec<Equation> {
    let mut equations = vec![];
    for line in text.split('\n') {
        let mut first_split = line.split(':');
        let equation_result = first_split.next().unwrap().parse::<u64>().unwrap();
        let equation_values = first_split
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|val_str| val_str.parse::<u64>().unwrap())
            .collect();
        equations.push(Equation {
            result: equation_result,
            values: equation_values,
        });
    }

    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_operator_variations() {
        assert_eq!(
            generate_operator_variations(1),
            vec![vec![Operator::Addition], vec![Operator::Multiplication], vec![Operator::Concatenation]]
        );
        assert_eq!(
            generate_operator_variations(2),
            vec![
                vec![Operator::Addition, Operator::Addition],
                vec![Operator::Addition, Operator::Multiplication],
                vec![Operator::Addition, Operator::Concatenation],
                vec![Operator::Multiplication, Operator::Addition],
                vec![Operator::Multiplication, Operator::Multiplication],
                vec![Operator::Multiplication, Operator::Concatenation],
                vec![Operator::Concatenation, Operator::Addition],
                vec![Operator::Concatenation, Operator::Multiplication],
                vec![Operator::Concatenation, Operator::Concatenation],
            ]
        );
    }

    #[test]
    // 190: 10 19
    fn test_equation_is_valid_true() {
        let equation = Equation {
            result: 190,
            values: vec![10, 19],
        };
        assert_eq!(equation_is_valid(&equation), true)
    }

    #[test]
    // 161011: 16 10 13
    fn test_equation_is_valid_false() {
        let equation = Equation {
            result: 161011,
            values: vec![16, 10, 13],
        };
        assert_eq!(equation_is_valid(&equation), false)
    }

    #[test]
    // 192: 17 8 14
    fn test_total_calibration_result() {
        let text = "190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20";
        let equations = string_to_equations(text.to_string());
        assert_eq!(total_calibration_result(&equations), 11387c)
    }
}
