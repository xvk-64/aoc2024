use std::fs;
use std::path::Path;

fn is_safe(report: &[u32], tolerate: bool) -> bool {
    let mut entry_iter = report.iter().peekable();

    let mut is_increasing = true;
    let mut is_decreasing = true;

    let mut is_in_range = true;

    while let Some( &level) = entry_iter.next() {
        if let Some(&&next) = entry_iter.peek() {
            if level <= next {
                is_increasing = false;
            }
            if level >= next {
                is_decreasing = false;
            }

            if !(is_increasing || is_decreasing) {
                if tolerate {
                    let without_level: Vec<u32> = report.iter().cloned().filter(|&x| x != level).collect();
                    if is_safe(&without_level, false) {
                        return true;
                    }
                } else {
                    return false;
                }
            }

            let diff = level.abs_diff(next);

            if diff < 1 || diff > 3 {
                is_in_range = false;

                if tolerate {
                    let without_level: Vec<u32> = report.iter().cloned().filter(|&x| x != level).collect();
                    if is_safe(&without_level, false) {
                        return true;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    return (is_increasing || is_decreasing) && is_in_range;
}

fn main() {
    let file_path = Path::new("inputs/day02.txt");
    let contents = fs::read_to_string(file_path).unwrap();

    let mut num_safe: u32 = 0;

    for line in contents.lines() {
        let report: Vec<u32> = line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect();

        if is_safe(&report, true) {
            num_safe += 1;
        }
    }

    println!("Part 1, Num safe: {}", num_safe);
}