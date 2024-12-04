use std::fs;
use std::path::Path;

fn add_within_matrix(matrix: &Vec<Vec<char>>, mut pos: (usize, usize), dir: &(isize, isize)) -> Option<(usize, usize)> {
    let new_y = pos.1 as isize + dir.1;
    let new_x = pos.0 as isize + dir.0;

    if new_x < 0 || new_y < 0 {return None;}

    pos.1 = new_y as usize;
    pos.0 = new_x as usize;

    if pos.1 >= matrix.len() || pos.0 >= matrix[pos.1].len() {return None;}

    Some(pos)
}

fn traverse_str(matrix: &Vec<Vec<char>>, s: &str, pos: &(usize, usize), dir: (isize, isize)) -> bool {
    let mut pos = pos.clone();

    for c in s.chars() {
        if matrix[pos.1][pos.0] != c {return false;}

        if c == s.chars().last().unwrap() {return true;}

        if let Some(new_pos) = add_within_matrix(matrix, pos, &dir) {
            pos = new_pos;
        } else {return false;}
    }

    true
}

fn add_dirs(v1: (isize, isize), v2: (isize, isize)) -> (isize, isize) {
    ((v1.0 + v2.0).clamp(-1, 1), (v1.1 + v2.1).clamp(-1, 1))
}
fn negate_dir(v: &(isize, isize)) -> (isize, isize) {
    (-v.0, -v.1)
}

fn get_orthogonal_dir(dir: &(isize, isize)) -> (isize, isize) {
    (dir.1, -dir.0)
}

fn find_mas(matrix: &Vec<Vec<char>>, pos: &(usize, usize), dir: &(isize, isize)) -> bool {
    if matrix[pos.1][pos.0] != 'A' {return false;}

    if let Some(prev_pos) = add_within_matrix(matrix, pos.clone(), &negate_dir(&dir)) {
        if matrix[prev_pos.1][prev_pos.0] != 'M' {return false;}
    } else {return false;}
    if let Some(prev_pos) = add_within_matrix(matrix, pos.clone(), &dir) {
        if matrix[prev_pos.1][prev_pos.0] != 'S' {return false;}
    } else {return false;}

    true
}

fn find_x_mas(matrix: &Vec<Vec<char>>, pos: &(usize, usize), up_dir: (isize, isize)) -> bool {
    if !(find_mas(matrix, &pos, &up_dir) || find_mas(matrix, &pos, &negate_dir(&up_dir))) {return false;}

    let orth_dir = get_orthogonal_dir(&up_dir);

    if !(find_mas(matrix, &pos, &orth_dir) || find_mas(matrix, &pos, &negate_dir(&orth_dir))) {return false;}

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
            if matrix[row][col] == 'A' {
                let mut num: u8 = 0;
                // if find_x_mas(&matrix, &(col, row), (0, 1)) {num+=1;}
                if find_x_mas(&matrix, &(col, row), (1, 1)) {num+=1;}

                matrix[row][col] = num.to_string().chars().nth(0).unwrap();

                num_xmas += num as usize;
            }
        }
        println!("{}", matrix[row].iter().collect::<String>());
    }

    println!("Num X-MAS: {}", num_xmas);
}