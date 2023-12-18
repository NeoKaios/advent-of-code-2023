use std::{fs, env, cmp::Reverse};
use priority_queue::PriorityQueue;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content, (0,3));
    compute(&content, (4,10));
}

fn compute(content: &String, constraints: (u8,u8)) {
    let lines = content.lines();
    let mut grid: Vec<Vec<u8>> = vec!();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars.iter().map(|ch| {
            if !ch.is_ascii_digit() {
                panic!("Found a no digit chat {ch}");
            } else {
                *ch as u8 - 48
            }
        }).collect());
    }
    // _print_grid(&grid);
    let res = shortest_path(&grid, constraints);
    println!("shortest is: {res}");
}

fn shortest_path(grid: &Vec<Vec<u8>>, contraints: (u8, u8)) -> i32 {
    let min_dist = contraints.0;
    let nb_row = grid.len();
    let nb_col = grid[0].len();
    let mut queue: PriorityQueue<(usize, usize, Dir), Reverse<i32>> = PriorityQueue::new();
    queue.push((0,0, Dir::Up(0)), Reverse(0));
    queue.push((0,0, Dir::Left(0)), Reverse(0));
    while !queue.is_empty() {
        match queue.pop() {
            None => panic!("Should not be here"),
            Some(item) => {
                // println!("{:?}", item);
                let ((r,c,d), p) = item;
                if r == nb_row-1 && c == nb_col-1 {
                    if d.get_len() < min_dist {
                        continue;
                    }
                    return p.0;
                }
                let mut next = vec!();
                get_next_nodes(item.0, nb_row, nb_col, contraints, &mut next);
                // println!("{:?}", next);

                for n in next {
                    let raw_p = p.0 + grid[n.0][n.1] as i32;
                    if let Some((_,next_p)) = queue.get(&n) {
                        if next_p.0 > raw_p {
                            println!("Prio changed");
                            queue.change_priority(&n, Reverse(raw_p));
                        }
                    } else {
                        queue.push(n, Reverse(raw_p));
                    }
                }

            }
        }
    }
    0
}

fn get_next_nodes(coord: (usize,usize,Dir), nb_row: usize, nb_col: usize, constraints: (u8,u8), out: &mut Vec<(usize,usize,Dir)>) {
    let (row,col,dir) = coord;
    let (min_dist,max_dist) = constraints;
    let keep_going = if dir.get_len() < min_dist {
        match dir {
            Dir::Up(_) => is_up,
            Dir::Right(_) => is_right,
            Dir::Left(_) => is_left,
            Dir::Down(_) => is_down,
        }
    } else {
        is_all
    };
    if !dir.is_down() && row != nb_row-1 && dir != Dir::Up(max_dist) && keep_going(Dir::Up(0)) { // explore up
        let d = if dir.is_up() {
                dir.inc()
        } else {
            Dir::Up(1)
        };
        out.push((row+1, col, d));
    }
    if !dir.is_right() && col != nb_col-1 && dir != Dir::Left(max_dist) && keep_going(Dir::Left(0)) { // explore left
        let d = if dir.is_left() {
                dir.inc()
        } else {
            Dir::Left(1)
        };
        out.push((row, col+1, d));
    }
    if !dir.is_left() && col != 0 && dir != Dir::Right(max_dist) && keep_going(Dir::Right(0)) { // explore right
        let d = if dir.is_right() {
                dir.inc()
        } else {
            Dir::Right(1)
        };
        out.push((row, col-1, d));
    }
    if !dir.is_up() && row != 0 && dir != Dir::Down(max_dist) && keep_going(Dir::Down(0)) { // explore down
        let d = if dir.is_down() {
                dir.inc()
        } else {
            Dir::Down(1)
        };
        out.push((row-1, col, d));
    }
}

fn is_up(dir: Dir) -> bool { dir.is_up() }
fn is_down(dir: Dir) -> bool { dir.is_down() }
fn is_right(dir: Dir) -> bool { dir.is_right() }
fn is_left(dir: Dir) -> bool { dir.is_left() }
fn is_all(_: Dir) -> bool { true }

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Dir {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
}

impl Dir {
    fn get_len(&self) -> u8 {
        match self {
            Dir::Up(i) => *i,
            Dir::Down(i) => *i,
            Dir::Right(i) => *i,
            Dir::Left(i) => *i,
        }
    }

    fn inc (&self) -> Dir {
        match self {
            Dir::Up(i) => Dir::Up(i+1),
            Dir::Down(i) => Dir::Down(i+1),
            Dir::Right(i) => Dir::Right(i+1),
            Dir::Left(i) => Dir::Left(i+1),
        }
    }
    fn is_up(&self) -> bool {
        if let Dir::Up(_) = self { true } else {false}
    }
    fn is_down(&self) -> bool {
        if let Dir::Down(_) = self { true } else {false}
    }
    fn is_right(&self) -> bool {
        if let Dir::Right(_) = self { true } else {false}
    }
    fn is_left(&self) -> bool {
        if let Dir::Left(_) = self { true } else {false}
    }
}

fn _print_grid(grid: &Vec<Vec<u8>>) {
    for r in grid {
        for ch in r {
            print!("{ch}");
        }
        print!("\n");
    }
}
