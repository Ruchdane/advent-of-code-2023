use std::fs;

const NUMBER_MAPPING: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("six", 6),
    ("five", 5),
    ("four", 4),
    ("nine", 9),
    ("three", 3),
    ("seven", 7),
    ("eight", 8),
];

fn main() {
    let contents = fs::read_to_string("data/calibration.txt").unwrap();
    let calibration = calibration_document(contents);
    println!("Calibration {}", calibration)
}

fn first_digit(line: &str) -> u32 {
    for n in 0..line.len() {
        if let Some(value) = line[n..].chars().next().unwrap().to_digit(10) {
            return value;
        }
        for (number, value) in NUMBER_MAPPING {
            let span = number.len();
            if n + span <= line.len() {
                let sub_line = line[n..(n + span)].to_string();
                if sub_line == *number {
                    return value;
                }
            };
        }
    }
    0
}
fn last_digit(line: &str) -> u32 {
    for n in (0..line.len()).rev() {
        if let Some(value) = line[n..].chars().next().unwrap().to_digit(10) {
            return value;
        }
        for (number, value) in NUMBER_MAPPING {
            let span = number.len();
            if n + 1 >= span && n < line.len() {
                let sub_result: String = line[((n + 1) - span)..=n].to_string();
                // println!("\t#{}:{} -> {}", n, sub_result, number);
                if sub_result == *number {
                    return value;
                }
            };
        }
    }
    0
}

pub fn calibration_document(lines: String) -> u32 {
    lines
        .split('\n')
        // .map(convert_spelled_value)
        .map(|line| {
            let first = first_digit(line);
            let last = last_digit(line);
            println!("{}:{}{}", line, first, last);
            first * 10 + last
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or(0)
}
