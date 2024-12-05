use std::collections::VecDeque;
use std::fs;
use std::mem::swap;
use std::path::Path;

fn main() {
    // Ordering rules
    let ordering_path = Path::new("inputs/day05-1.txt");
    let ordering_contents = fs::read_to_string(ordering_path).unwrap();

    // Updates
    let updates_path = Path::new("inputs/day05-2.txt");
    let updates_contents = fs::read_to_string(updates_path).unwrap();

    let mut all_nodes: Vec<usize> = Vec::new();
    let mut all_referenced: Vec<usize> = Vec::new();

    let mut pending_edges: Vec<(usize, usize)> = ordering_contents.lines().map(|l| {
        let (before, after) = l.split_once("|").unwrap();

        let edge = (after.parse::<usize>().unwrap(), before.parse::<usize>().unwrap());

        if !all_nodes.contains(&edge.0) {all_nodes.push(edge.0)}
        if !all_nodes.contains(&edge.1) {all_nodes.push(edge.1)}

        if !all_referenced.contains(&edge.1) {all_referenced.push(edge.1)}

        edge
    }).collect();

    let mut all_unreferenced = all_nodes.iter().filter(|n| !all_referenced.contains(n)).collect::<Vec<&usize>>();
    println!("{all_unreferenced:?}");
    println!("{all_referenced:?}");
    println!("{all_nodes:?}");

    let mut sorted: Vec<&usize> = Vec::new();

    while let Some(n) = all_unreferenced.pop() {
        sorted.push(n);

        let outgoing: Vec<(usize, &(usize, usize))> = pending_edges.iter().enumerate().filter(|e| e.0 == *n).collect();
        let mut to_delete = Vec::new();

        for (i, (_, m)) in outgoing {
            if pending_edges.iter().find(|(s, d)| d == m && s != n).is_none() {
                all_unreferenced.push(m);
            }

            to_delete.push(i);
        }

        // for i in to_delete {
        //     pending_edges.remove(i);
        // }
    }

    println!("{sorted:?}");
}