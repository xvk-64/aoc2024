use std::fs;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;
use regex::Regex;

fn consume_mul(c: Peekable<Chars>) -> (Peekable<Chars>, Option<u64>) {
    let mut p = c.clone();

    if p.next() != Some('m') {return (c, None);}
    if p.next() != Some('u') {return (c, None);}
    if p.next() != Some('l') {return (c, None);}
    if p.next() != Some('(') {return (c, None);}

    let mut arg1 = String::new();
    while let Some(d) = p.peek() {
        if !d.is_numeric() {break;}
        if arg1.len() == 3 {break;}

        arg1.push(p.next().unwrap());
    }

    if p.next() != Some(',') {return (c, None);}

    let mut arg2 = String::new();
    while let Some(d) = p.peek() {
        if !d.is_numeric() {break;}
        if arg2.len() == 3 {break;}

        arg2.push(p.next().unwrap());
    }

    if p.next() != Some(')') {return (c, None);}

    (p, Some(arg1.parse::<u64>().unwrap() * arg2.parse::<u64>().unwrap()))
}

fn consume_do(c: Peekable<Chars>) -> (Peekable<Chars>, Option<()>) {
    let mut p = c.clone();

    if p.next() != Some('d') {return (c, None);}
    if p.next() != Some('o') {return (c, None);}
    if p.next() != Some('(') {return (c, None);}
    if p.next() != Some(')') {return (c, None);}

    (p, Some(()))
}
fn consume_dont(c: Peekable<Chars>) -> (Peekable<Chars>, Option<()>) {
    let mut p = c.clone();

    if p.next() != Some('d') {return (c, None);}
    if p.next() != Some('o') {return (c, None);}
    if p.next() != Some('n') {return (c, None);}
    if p.next() != Some('\'') {return (c, None);}
    if p.next() != Some('t') {return (c, None);}
    if p.next() != Some('(') {return (c, None);}
    if p.next() != Some(')') {return (c, None);}

    (p, Some(()))
}

fn main() {
    let file_path = Path::new("inputs/day03.txt");
    let contents = fs::read_to_string(file_path).unwrap();


    let mut result: u64 = 0;

    let mut chars = contents.chars().peekable();

    let mut enabled = true;

    while let Some(_) = chars.peek() {
        let (i, r) = consume_do(chars);
        chars = i;
        if let Some(d) = r {
            enabled = true;
            continue;
        }
        let (i, r) = consume_dont(chars);
        chars = i;
        if let Some(d) = r {
            enabled = false;
            continue;
        }
        let (i, r) = consume_mul(chars);
        chars = i;
        if let Some(d) = r {
            if enabled {
                result += d;
            }
            continue;
        }

        chars.next();
    }

    // 192767529, 104083373
    println!("Result: {}", result);
}