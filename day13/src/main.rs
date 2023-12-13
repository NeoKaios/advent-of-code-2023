use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

fn check_mirror(lines: &Vec<&str>, mirror_line: usize)-> bool {
    let mut count = 0;
    for i in (0..mirror_line).rev() {
        if let Some(line_c) = lines.get(count+mirror_line) {
            if lines[i] != *line_c {
                return false;
            }
            count +=1;
        } else {
            break;
        }
    }
    return true;
}

fn detect_mirror(lines: &Vec<&str>)-> Option<usize> {
    for mirror_line in 1..lines.len(){
        if check_mirror(lines, mirror_line) {
            return Some(mirror_line);
        }
    }
    return None;
}

fn compute(content: &String) {
    let lines = content.lines();
    let mut grid: Vec<Vec<char>> = vec!();
    let mut rows: Vec<&str> = vec!();

    let n = content.lines().count();
    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;
    for (i,line) in lines.enumerate() {
        if !line.is_empty() {
            rows.push(line);
            let chars: Vec<char> = line.chars().collect();
            if grid.is_empty() {
                for ch in chars {
                    grid.push(vec![ch; 1]);
                }
            } else {
                for (i,g) in grid.iter_mut().enumerate() {
                    g.push(chars[i]);
                }
            }
        }
        if line.is_empty() || i == n-1 {
            let mut cols: Vec<String> = vec!();
            for g in &grid {
                cols.push(g.iter().collect::<String>());
            }
            let str_cols = cols.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            if let Some(r) = detect_mirror(&rows) {
                sum_1 += (r as u64)*100;
            } else if let Some(c) = detect_mirror(&str_cols) {
                sum_1 += c as u64;
            } else {
                panic!("No clean mirror found here")
            }
            if let Some(r) = detect_smudged_mirror(&rows) {
                sum_2 += (r as u64)*100;
            } else if let Some(c) = detect_smudged_mirror(&str_cols) {
                sum_2 += c as u64;
            } else {
                panic!("No smudged mirror found here")
            }
            grid.clear();
            cols.clear();
            rows.clear();
            continue;
        }
    }
    println!("The total number with clean mirrors is: {sum_1}");
    println!("The total number with smudged mirrors is: {sum_2}");
}

fn smudge_eq(one: &str, two: &str, old_can_smudge: bool) -> (bool, bool) {
    let mut can_smudge = old_can_smudge;
    for (o,t) in one.chars().zip(two.chars()) {
        if o != t && !can_smudge {
            return (false, old_can_smudge);
        }
        can_smudge &= o==t;
    }
    (true, can_smudge)
}

fn check_smudged_mirror(lines: &Vec<&str>, mirror_line: usize)->bool {
    let mut count = 0;
    let mut can_smudge = true;
    for i in (0..mirror_line).rev() {
        if let Some(line_c) = lines.get(count+mirror_line) {
            let (eq, can) = smudge_eq(lines[i], line_c, can_smudge);
            can_smudge = can;
            if !eq {
                return false;
            }
            count +=1;
        } else {
            break;
        }
    }
    return !can_smudge;
}

fn detect_smudged_mirror(lines: &Vec<&str>)-> Option<usize> {
    for mirror_line in 1..lines.len(){
        if check_smudged_mirror(lines, mirror_line) {
            return Some(mirror_line);
        }
    }
    return None;
}

