use std::fs;
use std::env;
use regex::Regex;
use std::panic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

const A_CHAR: char = 'x';
const B_CHAR: char = 'o';
fn second_part(content: &String) {
    // let nb_line = content.lines().count();
    let nb_column = content.lines().next().map(|s| s.len()).expect("Input file should have text");
    let formatted_content = content.replace('\n', "");
    let pipes = formatted_content.as_bytes();
    let mut inout: Vec<char> = formatted_content.chars().collect();
    let mut index: usize = 0;
    for (idx,pipe) in pipes.iter().enumerate() {
        if *pipe == b'S' {
            index = idx;
            break;
        }
    }
    let start = find_start_dir(pipes, index, nb_column);
    index = start.0;
    let mut dir = start.1;

    inout[index]='#';
    // first loop, setup inout with '#'
    while pipes[index] != b'S' {
        dir = dir_to_follow(pipes[index], dir).expect("Ill-formed input");
        index = add_dir(index, dir, nb_column);
        inout[index] = '#';
    }

    // second loop, define a/b sides
    index = start.0;
    dir = start.1;
    while pipes[index] != b'S' {
        // println!("Handling {} {index} {:?}", pipes[index] as char, dir);
        let (index1,index2, flag) = get_inout_index(pipes[index], dir, index, nb_column, &inout);
        let mut char1 = A_CHAR;
        let mut char2 = B_CHAR;
        if flag == b'a' {
            char2 = A_CHAR;
        } else if flag == b'b' {
            char1= B_CHAR;
        }
        if let Some(idx1) = index1 {
            flood(idx1, char1, &mut inout, nb_column);
        }
        if let Some(idx2) = index2 {
            flood(idx2, char2, &mut inout, nb_column);
        }
        dir = dir_to_follow(pipes[index], dir).expect("Ill-formed input");
        index = add_dir(index, dir, nb_column);
    }


    let mut a_count = 0;
    let mut b_count = 0;
    for ch in inout {
        if ch == 'x' {
            a_count +=1;
        }
        else if ch == 'o' {
            b_count +=1;
        }
    }
    println!("\nThe A zone counts {a_count} elements, the B zone counts {b_count} elements");
}


fn flood(index: usize, char: char, inout: &mut Vec<char>, nb_column: usize) {
    let cur = inout[index];
    if (cur == A_CHAR && char == B_CHAR) || (cur == B_CHAR && char == A_CHAR) {
        panic!("Flooding wrong zone");
    }
    if inout[index] == '#' || inout[index] == char {
        return;
    }
    inout[index] = char;

    let coord = to_coord(index, nb_column);
    if coord.0 != 0 {
        let next_index = add_dir(index, Dir::Up, nb_column);
        flood(next_index, char, inout, nb_column);
    }
    if coord.1 != 0 {
        let next_index = add_dir(index, Dir::Left, nb_column);
        flood(next_index, char, inout, nb_column);
    }
    if coord.1 != nb_column -1 {
        let next_index = add_dir(index, Dir::Right, nb_column);
        flood(next_index, char, inout, nb_column);
    }
    let down_index = add_dir(index, Dir::Down, nb_column);
    if inout.get(down_index).is_some() {
        flood(down_index, char, inout, nb_column);
    }
}

/// Return the indexes of either zone A or zone B, depending on the flag
/// Flag:
///  - b'a': both indexes belong to zone A
///  - b'b': both indexes belong to zone B
///  - b'x': first index belongs to zone A, second to zone B
fn get_inout_index(pipe: u8, dir: Dir, index: usize, nb_column: usize, inout: &Vec<char>) -> (Option<usize>,Option<usize>,u8) {
    let left = panic::catch_unwind(|| {add_dir(index, Dir::Left, nb_column)}).ok();
    let up = panic::catch_unwind(|| {add_dir(index, Dir::Up, nb_column)}).ok();
    let down_idx = add_dir(index, Dir::Down, nb_column);
    let down = inout.get(down_idx).map(|_| down_idx);
    let right_idx = add_dir(index, Dir::Right, nb_column);
    let right = inout.get(right_idx).map(|_| right_idx);
    match (pipe,dir) {
        (b'|', Dir::Up) => (left,right, b'x'),
        (b'|', Dir::Down) => (right,left, b'x'),
        (b'F', Dir::Up) => (left,up, b'a'),
        (b'F', Dir::Left) => (left, up, b'b'),
        (b'-', Dir::Right) => (up, down, b'x'),
        (b'-', Dir::Left) => (down, up, b'x'),
        (b'7', Dir::Right) => (up, right, b'a'),
        (b'7', Dir::Up) => (up, right, b'b'),
        (b'J', Dir::Down) => (right, down, b'a'),
        (b'J', Dir::Right) => (right, down, b'b'),
        (b'L', Dir::Left) => (down, left, b'a'),
        (b'L', Dir::Down) => (down, left, b'b'),
        _ => panic!("Unknown pipe/dir combination")
    }
}

fn dir_to_follow(pipe: u8, origin: Dir) -> Option<Dir>
{
    match (pipe, origin) {
        (b'-', Dir::Left) => Some(Dir::Left),
        (b'-', Dir::Right) => Some(Dir::Right),
        (b'|', Dir::Up) => Some(Dir::Up),
        (b'|', Dir::Down) => Some(Dir::Down),
        (b'F', Dir::Up) => Some(Dir::Right),
        (b'F', Dir::Left) => Some(Dir::Down),
        (b'J', Dir::Down) => Some(Dir::Left),
        (b'J', Dir::Right) => Some(Dir::Up),
        (b'L', Dir::Down) => Some(Dir::Right),
        (b'L', Dir::Left) => Some(Dir::Up),
        (b'7', Dir::Up) => Some(Dir::Left),
        (b'7', Dir::Right) => Some(Dir::Down),
        _ => None,
    }
}

fn add_dir(index: usize, dir: Dir, nb_column: usize) -> usize {
    let coord = to_coord(index, nb_column);
    let new_coord = match dir {
        Dir::Up => (coord.0-1, coord.1),
        Dir::Down => (coord.0+1, coord.1),
        Dir::Right => (coord.0, coord.1+1),
        Dir::Left => (coord.0, coord.1-1),
    };
    from_coord(new_coord, nb_column)
}

fn from_coord(coord: (usize,usize), nb_column: usize) -> usize {
    coord.0*nb_column + coord.1
}
fn to_coord(index: usize, nb_column: usize) -> (usize,usize) {
    (index/nb_column,index%nb_column)
}

fn find_start_dir(pipes: &[u8], index: usize, nb_column: usize) -> (usize, Dir) {
    let coord = to_coord(index, nb_column);
    if coord.0 != 0 {
        let dir = Dir::Up;
        let try_index = add_dir(index, dir, nb_column);
        if dir_to_follow(pipes[try_index], dir).is_some() {
            return (try_index, dir);
        }
    }
    if coord.1 != 0 {
        let dir = Dir::Left;
        let try_index = add_dir(index, dir, nb_column);
        if dir_to_follow(pipes[try_index], dir).is_some() {
            return (try_index, dir);
        }
    }
    if coord.1 != nb_column -1 {
        let dir = Dir::Right;
        let try_index = add_dir(index, dir, nb_column);
        if dir_to_follow(pipes[try_index], dir).is_some() {
            return (try_index, dir);
        }
    }
    panic!("Couldn't find a valid start for the loop");
}

fn first_part(content: &String) {
    // let nb_line = content.lines().count();
    let nb_column = content.lines().next().map(|s| s.len()).expect("Input file should have text");
    for line in content.lines() {

    }
    let formatted_content = content.replace('\n', "");
    let pipes = formatted_content.as_bytes();
    let mut index: usize = 0;
    for (idx,pipe) in pipes.iter().enumerate() {
        if *pipe == b'S' {
            index = idx;
            break;
        }
    }
    let start = find_start_dir(pipes, index, nb_column);
    index = start.0;
    let mut dir = start.1;
    let mut length_loop = 1;

    while pipes[index] != b'S' {
        dir = dir_to_follow(pipes[index], dir).expect("Ill-formed input");
        index = add_dir(index, dir, nb_column);
        length_loop+=1;
    }
    println!("The length of the loop is: {length_loop}, so furthest from start is: {} steps", length_loop/2);
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn _print_maze(maze: &Vec<char>, nb_column: usize) {
    let mut c =0;
    for ch in maze {
        if c%nb_column == 0 {
            print!("\n");
        }
        c+=1;
        print!("{ch}");
    }
    print!("\n");
}
