use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

fn second_part(content: &String) {
    let lines = content.lines();
    let mut instr = vec!();

    for line in lines {
        let (dir, len) = get_special_dir_and_len(line);
        instr.push((dir,len));
    }
    let mut area = 0;
    while instr.len() > 2 {
        let mut out = vec!();
        area+= simplify(&instr, &mut out);
        instr = out;
    }
    assert_eq!(instr[0].1, instr[1].1);
    println!("Hexadecimal lava laggon can hold: {} m3", area + instr[0].1 as i64 +1);
}

fn simplify(instr: &Vec<(Dir, i64)>, out: &mut Vec<(Dir, i64)>) -> i64 {
    let mut last_i = 0;
    let len = instr.len();
    let mut area: i64 = 0;
    let mut i = 0;
    while i < len-2 {
        let (i0,i1,i2) = (instr[i+0],instr[i+1],instr[i+2]);
        match (i0.0,i1.0,i2.0) {
            (Dir::Right, Dir::Down, Dir::Left) |  // remove bump
            (Dir::Left, Dir::Up, Dir::Right) |
            (Dir::Up, Dir::Right, Dir::Down) |
            (Dir::Down, Dir::Left, Dir::Up) => {
                    if i0.1 == i2.1 {
                        area+= (i1.1+1)*i2.1;
                        out.push(i1);
                    }
                    else if i0.1 > i2.1 {
                        area+= (i1.1+1)*i2.1;
                        out.push((i0.0, i0.1-i2.1));
                        out.push(i1);
                    } else {
                        area+= (i1.1+1)*i0.1;
                        out.push(i1);
                        out.push((i2.0, i2.1-i0.1));
                    }
                    i+=3;
                },
            (Dir::Right, Dir::Up, Dir::Left) | // remove dent
            (Dir::Left, Dir::Down, Dir::Right) |
            (Dir::Up, Dir::Left, Dir::Down) |
            (Dir::Down, Dir::Right, Dir::Up) => {
                    if i0.1 == i2.1 {
                        area-= (i1.1-1)*i2.1;
                        out.push(i1);
                    }
                    else if i0.1 > i2.1 {
                        area-= (i1.1-1)*i2.1;
                        out.push((i0.0, i0.1-i2.1));
                        out.push(i1);
                    } else {
                        area-= (i1.1-1)*i0.1;
                        out.push(i1);
                        out.push((i2.0, i2.1-i0.1));
                    }
                    i+=3;
                },
            (Dir::Up, Dir::Down, _) | //remove u-turn
            (Dir::Down, Dir::Up, _) |
            (Dir::Left, Dir::Right, _) |
            (Dir::Right, Dir::Left, _) => {
                    if i0.1 >= i1.1 {
                        area+= i1.1;
                        out.push((i0.0, i0.1-i1.1));
                    } else {
                        area+= i0.1;
                        out.push((i1.0, i1.1-i0.1));
                    }
                    i+=2;
            },
            (Dir::Up, Dir::Up, _) | // combine repetition
            (Dir::Down, Dir::Down, _) |
            (Dir::Right, Dir::Right, _) |
            (Dir::Left, Dir::Left, _) => {
                    out.push((i0.0, i0.1+i1.1));
                    i+=2;
            }
        _ => { out.push(i0); i+=1;},
        };
        last_i = i;
    }
    for i in last_i..len {
        out.push(instr[i]);
    }
    return area;
}

fn first_part(content: &String) {
    let lines = content.lines();
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut instr = vec!();
    let mut max_row = 0;
    let mut min_row = 0;
    let mut max_col = 0;
    let mut min_col = 0;

    for line in lines {
        let (dir, len) = get_dir_and_len(line);
        instr.push((dir,len));
        let len = len as i32;
        match dir {
            Dir::Right => col+=len,
            Dir::Left => col-=len,
            Dir::Up => row-=len,
            Dir::Down => row+=len,
        }
        max_row = std::cmp::max(max_row, row);
        max_col = std::cmp::max(max_col, col);
        min_row = std::cmp::min(min_row, row);
        min_col = std::cmp::min(min_col, col);
    }
    let mut grid = vec![vec!['.'; (max_col-min_col+3) as usize]; (max_row-min_row+3) as usize];
    let coord = (-min_row as usize +1, -min_col as usize +1);
    draw_pattern(&mut grid, &instr, coord);
    // _print_grid(&grid);
    let sum = count_area(&mut grid);
    println!("Laggon can hold: {sum} m3");
}

fn count_area(grid: &mut Vec<Vec<char>>) -> i32 {
    let nb_row = grid.len();
    let nb_col = grid[0].len();
    flood(grid, 0,0, nb_row, nb_col);
    grid.iter().fold(0, |acc, line| {
        acc + line.iter().filter(|ch| **ch != 'x').count() as i32
    })
}

fn draw_pattern(grid: &mut Vec<Vec<char>>, instr: &Vec<(Dir, u8)>, coord: (usize,usize)) {
    let mut row = coord.0;
    let mut col = coord.1;
    for (dir, len) in instr {
        for _ in 0..*len {
            match dir {
                Dir::Right => col+=1,
                Dir::Left => col-=1,
                Dir::Up => row-=1,
                Dir::Down => row+=1,
            }
            grid[row][col] = '#';
        }
    }
}

fn flood(grid: &mut Vec<Vec<char>>, row: usize, col: usize, nb_row: usize, nb_col: usize) {
    if grid[row][col] == 'x' || grid[row][col] == '#' { return; }
    grid[row][col] = 'x';
    if row != 0 {flood(grid, row-1, col, nb_row, nb_col);}
    if col != 0 {flood(grid, row, col-1, nb_row, nb_col);}
    if row != nb_row-1 {flood(grid, row+1, col, nb_row, nb_col);}
    if col != nb_col-1 {flood(grid, row, col+1, nb_row, nb_col);}
}

fn get_special_dir_and_len(s: &str) -> (Dir, i64){
    let mut split = s.split_whitespace();
    split.next();
    split.next();
    let (dir,num) = if let Some(s) = split.next() {
        let dir = match &s[7..8] {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => panic!("Unknown direction"),
        };
        let num = i64::from_str_radix(&s[2..7], 16).unwrap_or(0);
        (dir,num)
    } else {
        panic!("No direction");
    };
    return (dir,num);
}


fn get_dir_and_len(s: &str) -> (Dir, u8){
    let mut split = s.split_whitespace();
    let dir = if let Some(s) = split.next() {
        match s {
            "R" => Dir::Right,
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            _ => panic!("Unknown direction"),
        }
    } else {
        panic!("No direction");
    };
    let num = split.next().expect("No number").parse().unwrap_or(0);
    return (dir,num);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Right,
    Up,
    Down,
    Left,
}

fn _print_grid(grid: &Vec<Vec<char>>) {
    for r in grid {
        for ch in r {
            print!("{ch}");
        }
        print!("\n");
    }
}
