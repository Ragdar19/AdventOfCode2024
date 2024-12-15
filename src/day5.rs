use std::cmp::Ordering;

pub fn call_day5() -> () {
    let page_config = read_input();
    let (valid_updates, invalid_updates) = categorize_updates(&page_config);

    let sum_of_middle_pages_on_valid_updates = sum_middle_pages(&valid_updates);
    println!("Sum of middle pages on valid updates : {sum_of_middle_pages_on_valid_updates}");

    let fixed_updates = invalid_updates
        .iter()
        .map(|update| fix_invalid_update(update, &page_config.ordering_rules))
        .collect();
    let sum_of_middle_pages_on_invalid_updates = sum_middle_pages(&fixed_updates);
    println!("Sum of middle pages on invalid updates : {sum_of_middle_pages_on_invalid_updates}")
}

fn read_input() -> PageConfig {
    let mut ordering_rules = vec![];
    let mut updates = vec![];
    if let Ok(lines) = super::read_lines("input_day5.txt") {
        let mut reading_ordering_rules = true;
        for line in lines.flatten() {
            if line.is_empty() {
                reading_ordering_rules = false;
                continue;
            }

            match reading_ordering_rules {
                true => {
                    let splitted = line.split('|').collect::<Vec<&str>>();
                    ordering_rules.push((
                        splitted[0].parse::<i32>().unwrap(),
                        splitted[1].parse::<i32>().unwrap(),
                    ));
                }
                false => {
                    let splitted = line
                        .split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    updates.push(splitted);
                }
            }
        }
    }

    PageConfig {
        ordering_rules,
        updates,
    }
}

fn categorize_updates(page_config: &PageConfig) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut valid_updates = vec![];
    let mut invalid_updates = vec![];
    for update in &page_config.updates {
        let mut valid = true;
        for double_page in update.windows(2) {
            let first_page = double_page[0];
            let second_page = double_page[1];
            if !rule_exists(first_page, second_page, &page_config.ordering_rules) {
                valid = false;
                invalid_updates.push(update.clone());
                break;
            }
        }

        if valid {
            valid_updates.push(update.clone());
        }
    }

    (valid_updates, invalid_updates)
}

fn rule_exists(
    left_page_number: i32,
    right_page_number: i32,
    ordering_rules: &Vec<(i32, i32)>,
) -> bool {
    if let Some(_) = ordering_rules
        .iter()
        .find(|tuple| tuple.0 == left_page_number && tuple.1 == right_page_number)
    {
        return true;
    }

    return false;
}

fn get_middle_page(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}

fn sum_middle_pages(updates: &Vec<Vec<i32>>) -> i32 {
    updates.iter().map(|update| get_middle_page(update)).sum()
}

fn get_rules_for_update<'a>(
    update: &Vec<i32>,
    ordering_rules: &'a Vec<(i32, i32)>,
) -> Vec<&'a (i32, i32)> {
    ordering_rules
        .iter()
        .filter(|tuple| update.contains(&tuple.0) && update.contains(&tuple.1))
        .collect()
}

fn sort_ordering_rules(update: &Vec<i32>, ordering_rules: &Vec<&(i32, i32)>) -> Vec<i32> {
    let mut sorted_update = update.clone();
    sorted_update.sort_by(|a, b| {
        if ordering_rules.iter().any(|rule| rule.0 == *a && rule.1 == *b) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    sorted_update
}

fn fix_invalid_update(update: &Vec<i32>, ordering_rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let ordering_rules_for_this_update = get_rules_for_update(&update, &ordering_rules);
    let sorted_update = sort_ordering_rules(&update, &ordering_rules_for_this_update);

    sorted_update
}

struct PageConfig {
    ordering_rules: Vec<(i32, i32)>,
    updates: Vec<Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_updates() {
        let page_config = PageConfig {
            ordering_rules: vec![
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
            updates: vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        };

        assert_eq!(categorize_updates(&page_config).0.len(), 3);
    }

    #[test]
    fn test_get_middle_page() {
        let update = vec![75, 29, 13];
        assert_eq!(get_middle_page(&update), 29);
    }

    #[test]
    fn test_sum_middle_pages() {
        let updates = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];

        assert_eq!(sum_middle_pages(&updates), 143);
    }

    #[test]
    fn test_sort_ordering_rules() {
        let ordering_rules = vec![
            (47, 53),
            (97, 61),
            (97, 47),
            (75, 53),
            (61, 53),
            (97, 53),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61)
        ];
        let update = vec![75, 97, 47, 61, 53];

        assert_eq!(sort_ordering_rules(&update, &ordering_rules.iter().collect()), vec![97,75,47,61,53]);
    }

    #[test]
    fn test_fix_invalid_update() {
        let ordering_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let invalid_update1 = vec![75, 97, 47, 61, 53];
        let invalid_update2 = vec![61, 13, 29];
        let invalid_update3 = vec![97, 13, 75, 29, 47];

        assert_eq!(
            fix_invalid_update(&invalid_update1, &ordering_rules),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            fix_invalid_update(&invalid_update2, &ordering_rules),
            vec![61, 29, 13]
        );
        assert_eq!(
            fix_invalid_update(&invalid_update3, &ordering_rules),
            vec![97, 75, 47, 29, 13]
        );
    }
}