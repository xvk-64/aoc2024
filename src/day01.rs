use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let file_path = Path::new("inputs/day01.txt");
    let contents = fs::read_to_string(file_path);

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in contents.unwrap().split("\n") {
        let split: Vec<&str> = line.split_whitespace().collect();

        if split.len() != 2 {break;}

        list1.push(split[0].trim().parse::<i32>().unwrap());
        list2.push(split[1].trim().parse::<i32>().unwrap());
    }

    list1.sort();
    list2.sort();

    let diff = (0..list1.len()).map(|i| (list1[i] - list2[i]).abs()).sum::<i32>();

    println!("Part 1: Difference: {}", diff);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for elem in list2 {
        counts.entry(elem).and_modify(|e| *e += 1).or_insert(1);
    }

    let similarity = list1.iter().map(|elem| {
        elem * (counts.get(&elem).unwrap_or(&0))
    }).sum::<i32>();
    
    println!("Part 2: Similarity: {}", similarity);
}
