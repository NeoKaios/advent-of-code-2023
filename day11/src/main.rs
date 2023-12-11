use std::cmp;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content)
}

const SECOND_PART_MULTIPLIER: i32 = 1000000;
const FIRST_PART_MULTIPLIER: i32 = 2;

fn from_coord(coord: (usize,usize), nb_column: usize) -> usize {
    coord.0*nb_column + coord.1
}
fn to_coord(index: usize, nb_column: usize) -> (i32,i32) {
    ((index/nb_column) as i32,(index%nb_column) as i32)
}

fn compute(content: &String) {
    // let nb_line = content.lines().count();
    let nb_column = content.lines().next().map(|s| s.len()).expect("Input file should have text");
    let mut empty_lines = vec!();
    let mut empty_col = vec!();
    let mut galaxies = vec!();
    let mut nb_lines = 0;
    for (idx,line) in content.lines().enumerate() {
        nb_lines = idx+1;
        if !line.contains('#') {
            empty_lines.push(idx);
        }
    }
    let formatted_content = content.replace('\n', "");
    let universe = formatted_content.as_bytes();
    for c in 0..nb_column {
        let mut col_is_empty = true;
        for l in 0..nb_lines {
            let idx = from_coord((l,c), nb_column);
            if universe[idx] == b'#' {
                col_is_empty = false;
                galaxies.push(idx)
            }
        }
        if col_is_empty {
            empty_col.push(c);
        }
    }
    let len = galaxies.len();
    let mut sum_1: u64 = 0;
    let mut sum_2: u64 = 0;
    for i in 0..len {
        for j in (i+1)..len {
            let coord_i = to_coord(galaxies[i], nb_column);
            let coord_j = to_coord(galaxies[j], nb_column);
            let min_l = cmp::min(coord_i.0, coord_j.0);
            let max_l = cmp::max(coord_i.0, coord_j.0);
            let expanded_lines = empty_lines.iter().fold(0, |acc,idx| {
                if (min_l as usize) < *idx && *idx < max_l as usize {
                    return acc+1;
                }
                acc
            });
            let min_c = cmp::min(coord_i.1, coord_j.1);
            let max_c = cmp::max(coord_i.1, coord_j.1);
            let expanded_col = empty_col.iter().fold(0, |acc,idx| {
                if (min_c as usize) < *idx && *idx < max_c as usize {
                    return acc+1;
                }
                acc
            });
            let distance = (coord_i.0-coord_j.0).abs() + (coord_j.1-coord_i.1).abs();
            sum_1+=(distance + (expanded_col + expanded_lines)*(FIRST_PART_MULTIPLIER-1)) as u64;
            sum_2+=(distance + (expanded_col + expanded_lines)*(SECOND_PART_MULTIPLIER-1)) as u64;
        }
    }
    println!("Sum of distances (mult {FIRST_PART_MULTIPLIER}): {}", sum_1);
    println!("Sum of distances (mult {SECOND_PART_MULTIPLIER}): {}", sum_2);
}
