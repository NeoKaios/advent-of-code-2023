use std::collections::HashMap;
use std::fs;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

fn _check_line(line: &Vec<char>, num: &Vec<u8>) -> bool {
    let mut nu: Vec<u8> = vec!();
    let mut seq_break = true;
    for ch in line {
        match *ch {
        '.' => {
            seq_break = true;
        },
        '#' => {
            if seq_break {
                seq_break = false;
                nu.push(1u8);
            } else {
                if let Some(last) = nu.last_mut() {
                    *last+=1;
                }
            }
        },
        _ => panic!("Char '{ch}' unknown in check_line"),
        }
    }
    return nu == *num;
}

fn find_count(row: &[char], num: &[u8], cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    let n = *num.first().expect("No number") as usize;
    let numlen = num.len();
    let rowlen = row.len();
    let sum: u8 = num.iter().sum();
    if rowlen < (sum as usize) + numlen-1 {return 0;}
    if let Some(res) = cache.get(&(rowlen, numlen)) {
        return *res;
    }
    let mut arrangement_w_block_here = 0;
    let mut arrangement_w_block_later = 0;
    let mut can_place_full_block = false;
    let mut block_sep_index: usize = 0;
    let mut should_continue = false;
    for (idx,ch) in row.iter().enumerate() {
        if idx < n {
            if *ch == '.' {
                should_continue = true;
                break; //cannot put block here (not continuous)
            } else {
                if idx == n-1 {
                    can_place_full_block = true; //exact at last elem of block
                }
                continue;
            }
        } else if idx == n {
            if *ch == '#' {
                should_continue = true;
                break; //cannot put block here (no end)
            } else {
                block_sep_index = idx;
                break;
            }
        }
    }
    let can_continue = row[0] != '#';
    if should_continue {
        if can_continue {
            arrangement_w_block_later += find_count(&row[1..], num, cache);
        }
    } else if block_sep_index != 0 {
        if numlen >= 2 && row.get(block_sep_index+1).is_some() {
            arrangement_w_block_here = find_count(&row[block_sep_index+1..], &num[1..], cache);
        }
        else if numlen == 1 && can_be_empty(&row[block_sep_index+1..]) {
            arrangement_w_block_here = 1;
        }
        if can_continue {
            arrangement_w_block_later += find_count(&row[1..], num, cache);
        }
    } else if can_place_full_block && numlen==1 && !should_continue { //end of row
        arrangement_w_block_here = 1;
    }
    let total = arrangement_w_block_here+arrangement_w_block_later;
    cache.insert((rowlen, numlen), total);
    total
}

fn can_be_empty(row: &[char]) -> bool {
    for ch in row {
        if *ch == '#' {
            return false;
        }
    }
    return true;
}

fn first_part(content: &String) {
    let lines = content.lines();
    let total: u64 = lines.fold(0, |acc,line| {
        let splits: Vec<&str> = line.split_whitespace().into_iter().collect();
        let row: Vec<char> = splits.get(0).expect("Line without row").chars().collect();
        let num: Vec<u8> = splits.get(1).expect("Line without num").split(',').map(|s| s.parse().unwrap_or(0)).collect();
        let mut cache = HashMap::new();
        let tot = find_count(&row, &num, &mut cache);
        acc +tot
    });
    println!("The total number of arrangements is: {total}");
}

fn second_part(content: &String) {
    let lines = content.lines();
    let now = Instant::now();
    let total: u64 = lines.fold(0, |acc,line| {
        let splits: Vec<&str> = line.split_whitespace().into_iter().collect();
        let r = splits.get(0).expect("Line without row");
        let big_row = format!("{r}?{r}?{r}?{r}?{r}");
        let row: Vec<char> = big_row.chars().collect();
        let n = splits.get(1).expect("Line without number");
        let big_num = format!("{n},{n},{n},{n},{n}");
        let num: Vec<u8> = big_num.split(',').map(|s| s.parse().unwrap_or(0)).collect();
        let mut cache = HashMap::new();
        let tot = find_count(&row, &num, &mut cache) as u64;
        acc +tot
    });
    eprintln!("Part 2 in {}ms", now.elapsed().as_micros());
    println!("The total number of arrangements with folded record is: {total}");
}
