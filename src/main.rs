use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use day1::call_day1;
use day2::call_day2;
use day3::call_day3;
use day4::call_day4;
use day5::call_day5;
use day6::call_day6;
use day7::call_day7;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    // call_day1();
    // call_day2();
    // call_day3();
    // call_day4();
    // call_day5();
    // call_day6();
    call_day7();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
