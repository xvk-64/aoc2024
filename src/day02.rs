use std::fs;
use std::path::Path;

fn is_safe(report: &[u32]) -> bool {
    let mut entry_iter = report.iter().peekable();

    let mut is_increasing = true;
    let mut is_decreasing = true;

    while let Some( &level) = entry_iter.next() {
        if let Some(&&next) = entry_iter.peek() {
            if level <= next {
                is_increasing = false;
            }
            if level >= next {
                is_decreasing = false;
            }

            if !(is_increasing || is_decreasing) {
                return false;
            }

            let diff = level.abs_diff(next);

            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let file_path = Path::new("inputs/day02.txt");
    let contents = fs::read_to_string(file_path).unwrap();

    let mut num_safe: u32 = 0;

    for line in contents.lines() {
        let report: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

        if is_safe(&report) {
            num_safe += 1;
        }
    }

    println!("Part 1, Num safe: {}", num_safe);
}