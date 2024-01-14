use std::{fs, env, collections::HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

const STEPS_PART_1: usize = 64;
const STEPS_PART_2: usize = 26501365;

fn compute(content: &String) {
    let lines = content.lines();
    let mut grid: Vec<Vec<char>> = vec!();
    lines.for_each(|line| grid.push(line.chars().collect()));
    let edge = (grid.len()-1, grid[0].len()-1);
    let size = (grid.len(), grid[0].len());
    let start = (edge.0/2, edge.1/2);

    assert_eq!(grid[start.0][start.1], 'S');

    // ===== Part 1 =====
    println!("The total number of garden plot reachable in {STEPS_PART_1} steps is: {}", reachable_from(&grid, start, STEPS_PART_1, None));

    // ===== Part 2 =====
    let mut prior_len = 0;
    let mut prior_prior_len = 0;
    {
        // What oscillation
        let mut set: HashSet<(usize,usize)> = HashSet::new();
        set.insert(start);
        loop {
            set = set.iter().fold(HashSet::new(), |mut acc_set, coord| {
                for next in get_next(&grid, *coord, edge) {
                    acc_set.insert(next);
                }
                acc_set
            });
            if prior_prior_len == set.len() {
                if !set.contains(&start) {
                    let temp = prior_len;
                    prior_len = prior_prior_len;
                    prior_prior_len = temp; // contains start
                }
                break;
            }
            prior_prior_len = prior_len;
            prior_len = set.len();
        }
    }
    let oscillation_1 = prior_prior_len; // contains start
    let oscillation_2 = prior_len;

    {
        // origin (0,0)
        let res_origin = if STEPS_PART_2%2==0 {oscillation_1} else {oscillation_2};
        let plot_when_not_sync_start = oscillation_1+oscillation_2-res_origin;
        let plot_when_sync_start = res_origin;

        // quadrants: (x,y) | x!=0 && y!=0
        let offset = edge.0+2;
        let next_block = size.0;
        let fill_current = 2*edge.0;
        let max_full_norm = (STEPS_PART_2-offset-fill_current)/next_block+2;
        let mut res_4_quadrant = 4*plot_when_sync_start*(max_full_norm/2)*(max_full_norm/2) + 4*plot_when_not_sync_start*(max_full_norm-1)/2*(max_full_norm+1)/2;
        let remaing_steps = STEPS_PART_2-(offset+next_block*(max_full_norm-1));
        let first_unfull = (remaing_steps,max_full_norm);
        let second_unfull = if remaing_steps >= next_block {
            Some((remaing_steps-next_block,max_full_norm+1))
        } else {None};
        for start_pos in [
            (0,0),
            edge,
            (edge.0,0),
            (0,edge.1),
        ] {
            let mut set = HashSet::from([start_pos]);
            let mut first_steps = first_unfull.0;
            if let Some((steps,amount)) = second_unfull {
                res_4_quadrant += amount*reachable_from(&grid, start_pos, steps, Some(&mut set));
                first_steps-=steps;
            }
            res_4_quadrant += first_unfull.1*reachable_from(&grid, start_pos, first_steps, Some(&mut set));
        }

        // axis (x,0) & (0,y)
        let offset = edge.0/2+1;
        let next_block = size.0;
        let fill_current = edge.0+edge.0/2;
        let max_full_norm = (STEPS_PART_2-offset-fill_current)/next_block+1;
        let mut res_4_axis = 4*plot_when_sync_start*(max_full_norm/2)+4*plot_when_not_sync_start*(max_full_norm/2+max_full_norm%2);
        let remaing_steps = STEPS_PART_2-(offset+next_block*max_full_norm);
        let mut steps_arr = Vec::from([remaing_steps]);
        if remaing_steps >= next_block {
            steps_arr.push(remaing_steps-next_block);
        }
        for steps in steps_arr {
            for start_pos in [
                (start.0,0),
                (0,start.1),
                (edge.0,start.1),
                (start.0,edge.1),
            ] {
                res_4_axis += reachable_from(&grid, start_pos, steps, None);
            }
        }
        println!("The total number of garden plot reachable in {STEPS_PART_2} steps is: {}", res_4_quadrant+res_4_axis+res_origin);
    }
}

fn reachable_from(grid: &Vec<Vec<char>>, start_pos: (usize,usize), steps: usize, starting_set: Option<&mut HashSet<(usize,usize)>>) -> usize {
    let edge = (grid.len()-1, grid[0].len()-1);
    let mut one_set = HashSet::from([start_pos]);
    let set = if let Some(set) = starting_set {set} else { &mut one_set};
    for _ in 0..steps {
        *set = set.iter().fold(HashSet::new(), |mut acc_set, coord| {
            for next in get_next(&grid, *coord, edge) {
                acc_set.insert(next);
            }
            acc_set
        });
    }
    set.len()
}

fn get_next(grid: &Vec<Vec<char>>, coord: (usize, usize), edge: (usize,usize)) -> Vec<(usize,usize)> {
    let (row,col) = coord;
    let mut output = vec!();
    if row != 0 && grid[row-1][col] != '#' {
        output.push((row-1, col));
    }
    if col != 0 && grid[row][col-1] != '#' {
        output.push((row, col-1));
    }
    if row != edge.0 && grid[row+1][col] != '#' {
        output.push((row+1, col));
    }
    if col != edge.1 && grid[row][col+1] != '#' {
        output.push((row, col+1));
    }
    output
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for l in grid {
        for ch in l {
            print!("{ch}");
        }
        print!("\n");
    }
}
