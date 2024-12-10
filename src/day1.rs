
use std::collections::HashMap;

pub fn call_day1() -> () {
    let (mut list1, mut list2) = read_location_ids();

    list1.sort();
    list2.sort();

    let distance = distance_between_sorted_lists(&list1, &list2);
    let similarity_score = similarity_score(&list1, &list2);

    println!("Sum of distances : {distance}");
    println!("Similarity score : {similarity_score}");
}

fn read_location_ids() -> (Vec<i32>, Vec<i32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    if let Ok(lines) = super::read_lines("input_day1.txt") {
        for line in lines.flatten() {
            let splitted = line.split_whitespace().collect::<Vec<&str>>();

            if let Ok(location_id) = splitted[0].parse::<i32>() {
                list1.push(location_id);
            }

            if let Ok(location_id) = splitted[1].parse::<i32>() {
                list2.push(location_id);
            }
        }
    }

    (list1, list2)
}

fn distance_between_sorted_lists(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut sum_distances = 0;
    for i in 0..list1.len() {
        let distance = list1[i] - list2[i];
        sum_distances = sum_distances + distance.abs();
    }

    sum_distances
}

fn similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut similarity_score = 0;
    let number_occurences = number_occurences(list2);
    for i in list1 {
        if number_occurences.contains_key(&i) {
            similarity_score = similarity_score + number_occurences[&i] * i;
        }
    }

    similarity_score
}

fn number_occurences(list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut number_occurences = HashMap::new();
    for element in list {
        number_occurences
            .entry(*element)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    number_occurences
}