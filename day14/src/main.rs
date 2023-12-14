use std::{fs, env, collections::{HashMap, hash_map::DefaultHasher}, hash::{Hasher, Hash}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

const NB_CYCLE: i32 = 1000000000;
fn second_part(content: &String) {
    let lines = content.lines();
    let mut grid: Vec<Vec<char>> = vec!();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }
    let nb_lines = grid.len();
    let nb_column = grid[0].len();
    let mut cache: HashMap<(u64, i32), i32> = HashMap::new();

    let mut loop_size:i32 = 0;
    for i in 1.. {
        cycle(&mut grid, nb_lines, nb_column);
        if i > 100 {
            let hash = (get_hash(&grid), get_load(&grid, nb_lines));
            if let Some(last_seen) = cache.get(&hash) {
                loop_size = i-last_seen;
                break;
            }
            cache.insert(hash, i);
        }
    }
    for (k,v) in cache.iter(){
        if (NB_CYCLE-*v)%loop_size == 0 {
            println!("The total load after {NB_CYCLE} cyles is: {}", k.1);
        }
    }
}

fn first_part(content: &String) {
    let lines = content.lines();
    let mut grid: Vec<Vec<char>> = vec!();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }
    let nb_lines = grid.len();
    let nb_column = grid[0].len();
    north(&mut grid, nb_lines, nb_column);
    println!("The total load after a tilt north is: {}", get_load(&grid, nb_lines));
}

fn get_hash(grid: &Vec<Vec<char>>) -> u64 {
    let mut s = DefaultHasher::new();
    for l in grid {
        for  ch in l {
            (*ch).hash(&mut s);
        }
    }
    s.finish()
}

fn get_load(grid: &Vec<Vec<char>>, nb_lines: usize) -> i32 {
    return grid.iter().enumerate().fold(0, |acc,(i,line)| {
        let nb = line.iter().filter(|ch| **ch=='O').count();
        acc + (nb*(nb_lines-i)) as i32
    });
}

fn cycle(grid: &mut Vec<Vec<char>>, nb_lines: usize, nb_column: usize) {
    north(grid, nb_lines, nb_column);
    west(grid, nb_lines, nb_column);
    south(grid, nb_lines, nb_column);
    est(grid, nb_lines, nb_column);
}

fn north(grid: &mut Vec<Vec<char>>, nb_lines: usize, nb_column: usize) {
    for c in 0..nb_column {
        let mut got_match = true;
        while got_match {
            got_match = false;
            for l in 1..nb_lines {
                if grid[l][c] == 'O' && grid[l-1][c] == '.' {
                    grid[l-1][c] = 'O';
                    grid[l][c] = '.';
                    got_match = true;
                }
            }
        }
    }
}

fn south(grid: &mut Vec<Vec<char>>, nb_lines: usize, nb_column: usize) {
    for c in 0..nb_column {
        let mut got_match = true;
        while got_match {
            got_match = false;
            for l in 1..nb_lines {
                if grid[nb_lines-l-1][c] == 'O' && grid[nb_lines-l][c] == '.' {
                    grid[nb_lines-l][c] = 'O';
                    grid[nb_lines-l-1][c] = '.';
                    got_match = true;
                }
            }
        }
    }
}

fn west(grid: &mut Vec<Vec<char>>, nb_lines: usize, nb_column: usize) {
    for l in 0..nb_lines {
        let mut got_match = true;
        while got_match {
            got_match = false;
            for c in 1..nb_column {
                if grid[l][c] == 'O' && grid[l][c-1] == '.' {
                    grid[l][c-1] = 'O';
                    grid[l][c] = '.';
                    got_match = true;
                }
            }
        }
    }
}

fn est(grid: &mut Vec<Vec<char>>, nb_lines: usize, nb_column: usize) {
    for l in 0..nb_lines {
        let mut got_match = true;
        while got_match {
            got_match = false;
            for c in 1..nb_column {
                if grid[l][nb_column-c-1] == 'O' && grid[l][nb_column-c] == '.' {
                    grid[l][nb_column-c] = 'O';
                    grid[l][nb_column-c-1] = '.';
                    got_match = true;
                }
            }
        }
    }
}

fn _print_grid(maze: &Vec<Vec<char>>) {
    for l in maze {
        for ch in l {
            print!("{ch}");
        }
        print!("\n");
    }
}
