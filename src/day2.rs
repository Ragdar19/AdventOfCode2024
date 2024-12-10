pub fn call_day2() -> () {
    let reports = read_reports();
    let number_of_safe_reports = number_of_safe_reports(&reports);
    println!("Number of safe reports : {number_of_safe_reports}");
}

fn read_reports() -> Vec<Vec<i32>> {
    let mut reports = vec![];
    if let Ok(lines) = super::read_lines("input_day2.txt") {
        for line in lines.flatten() {
            let splitted = line
                .split_whitespace()
                .map(|element| element.parse().unwrap())
                .collect();
            reports.push(splitted);
        }
    }

    reports
}

fn number_of_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    let mut number_of_safe_reports = 0;
    for report in reports {
        if report_is_safe(&report) {
            number_of_safe_reports += 1;
        }
    }

    number_of_safe_reports
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    let accepted_range = 1..=3;
    let mut previous_level = report[0];
    let mut direction = None;
    let mut number_of_errors = 0;
    for i in 1..report.len() {
        let difference = previous_level - report[i];
        // For the first iteration
        if direction == None {
            if difference > 0 {
                direction = Some(ReportDirection::Decreasing);
            } else if difference < 0 {
                direction = Some(ReportDirection::Increasing);
            } else {
                number_of_errors += 1;
                previous_level = report[i];
                continue;
            }
        }

        if difference > 0 && direction != Some(ReportDirection::Decreasing) {
            number_of_errors += 1;
        } else if difference < 0 && direction != Some(ReportDirection::Increasing) {
            number_of_errors += 1;
        } else if !accepted_range.contains(&difference.abs()) {
            number_of_errors += 1;
        }

        previous_level = report[i];
    }

    return number_of_errors <= 1;
}

#[derive(PartialEq)]
enum ReportDirection {
    Increasing,
    Decreasing,
}
