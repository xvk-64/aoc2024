use std::fs;
use std::path::Path;

fn traverse_xmas(matrix: &Vec<Vec<char>>, pos: &(usize, usize), dir: (isize, isize)) -> bool {
    let mut pos = pos.clone();

    for c in "XMAS".chars() {
        if matrix[pos.1][pos.0] != c {return false;}
        
        if c == 'S' {return true;}

        let new_y = pos.1 as isize + dir.1;
        let new_x = pos.0 as isize + dir.0;

        if new_x < 0 || new_y < 0 {return false;}

        pos.1 = new_y as usize;
        pos.0 = new_x as usize;

        if pos.1 >= matrix.len() || pos.0 >= matrix[pos.1].len() {return false;}
    }

    true
}

fn main() {
    let file_path = Path::new("inputs/day04.txt");
    let contents = fs::read_to_string(file_path).unwrap();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    let mut num_xmas: usize = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'X' {
                let mut num: u8 = 0;
    
                if traverse_xmas(&matrix, &(col, row), (0, 1)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (1, 1)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (1, 0)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (1, -1)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (0, -1)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (-1, -1)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (-1, 0)) {num+=1;}
                if traverse_xmas(&matrix, &(col, row), (-1, 1)) {num+=1;}
    
                matrix[row][col] = num.to_string().chars().nth(0).unwrap();
    
                
                num_xmas += num as usize;
            }
        }
        println!("{}", matrix[row].iter().collect::<String>());
    }

    println!("Num XMAS: {}", num_xmas);
}