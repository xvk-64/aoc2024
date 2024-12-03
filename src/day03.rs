use std::fs;
use std::path::Path;
use std::str::Chars;

fn consume_mul(c: &mut Chars) -> u32 {
    if c.next() != Some('m') { return 0; }
    if c.next() != Some('u') { return 0; }
    if c.next() != Some('l') { return 0; }
    if c.next() != Some('(') { return 0; }

    let mut arg1 = String::new();
    while let Some(d) = c.next() {
        if d == ',' { break; }

        arg1.push(d);
    }
    if arg1.len() < 1 || arg1.len() > 3 { return 0;}


    let mut arg1 = String::new();
    while let Some(d) = c.next() {
        if d == ')' { break; }

        arg1.push(d);
    }
    if arg1.len() < 1 || arg1.len() > 3 { return 0;}

    0
}

fn main() {
    let file_path = Path::new("inputs/day03.txt");
    let contents = fs::read_to_string(file_path).unwrap();

    let mut i = contents.chars();

    if let Some(c) = i.next() {
        println!("{}", c);
    }
}