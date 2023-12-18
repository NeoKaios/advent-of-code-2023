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
    let mut grid: Vec<Vec<u8>> = vec!();
    for line in lines {
        grid.push(line.as_bytes().to_vec());
    }
    let edge = (grid.len(), grid[0].len());
    let mut max_energy = 0;
    let mut energy_map: Vec<Vec<u8>> = vec![vec![0; edge.1]; edge.0];
    for (col,dir) in [(0, Dir::R), (edge.1-1, Dir::L)] { // right and left sides
        for row in 0..edge.0 {
            diffuse_ray(&grid, &mut energy_map, Ray {row, col, dir}, edge);
            max_energy = std::cmp::max(max_energy, get_energy_and_clear(&mut energy_map));
        }
    }
    for (row,dir) in [(0, Dir::D), (edge.0-1, Dir::U)] { // top and bottom sides
        for col in 0..edge.1 {
            diffuse_ray(&grid, &mut energy_map, Ray {row, col, dir}, edge);
            max_energy = std::cmp::max(max_energy, get_energy_and_clear(&mut energy_map));
        }
    }
    println!("Maximized energy reaches: {max_energy}");
}

fn get_energy_and_clear(energy_map: &mut Vec<Vec<u8>>) -> usize {
    energy_map.iter_mut().fold(0, |acc, line| {
        let mut acc = acc;
        for u in line {
            if *u > 0 {
                *u = 0;
                acc+=1;
            }
        }
        acc
    })
}

fn first_part(content: &String) {
    let lines = content.lines();
    let mut grid: Vec<Vec<u8>> = vec!();
    for line in lines {
        grid.push(line.as_bytes().to_vec());
    }
    let edge = (grid.len(), grid[0].len());
    let mut energy_map: Vec<Vec<u8>> = vec![vec![0; edge.1]; edge.0];
    diffuse_ray(&grid, &mut energy_map, Ray {row: 0, col: 0, dir: Dir::R}, edge);
    println!("Energy with top-left horizontal laser is: {}", get_energy_and_clear(&mut energy_map));
}

fn get_next_coord(coord: (usize,usize), dir: Dir, edge: (usize, usize)) -> Option<(usize,usize)> {
    let (row,col) = coord;
    match dir {
        Dir::R => {
            if col == edge.1-1 { return None; }
            Some((row, col+1))
        }, Dir::L => {
            if col == 0 { return None; }
            Some((row, col-1))
        }, Dir::D => {
            if row == edge.0 - 1 { return None; }
            Some((row+1, col))
        }, Dir::U => {
            if row == 0 { return None; }
            Some((row-1, col))
        },
    }
}

fn get_dir_after(elem: u8, dir: Dir) -> (Dir, Option<Dir>) {
    match elem {
        b'.' => (dir, None),
        b'/' => (dir.flip_ru(), None),
        b'\\' => (dir.flip_rd(), None),
        b'|' => {
            match dir {
                Dir::R | Dir::L => (Dir::U, Some(Dir::D)),
                Dir::U | Dir::D => (dir, None)
            }
        },
        b'-' => {
            match dir {
                Dir::U | Dir::D => (Dir::R, Some(Dir::L)),
                Dir::R | Dir::L => (dir, None)
            }
        },
        _ => panic!("Unknown char on grid {}", elem),
    }
}

fn diffuse_ray(grid: &Vec<Vec<u8>>, energy_map: &mut Vec<Vec<u8>>, ray: Ray, edge: (usize,usize)) {
    // println!("{:?}", ray);
    if energy_map[ray.row][ray.col] & ray.dir.as_flag() > 0 {
        return;
    } else {
        energy_map[ray.row][ray.col] |= ray.dir.as_flag();
    }
    let (next_dir, opt_next_dir) = get_dir_after(grid[ray.row][ray.col], ray.dir);

    let next = get_next_coord((ray.row,ray.col), next_dir, edge);
    if let Some(next) = next {
        diffuse_ray(grid, energy_map, Ray { row: next.0, col: next.1, dir: next_dir}, edge);
    }
    if let Some(next_dir) = opt_next_dir {
        if let Some(next) = get_next_coord((ray.row,ray.col), next_dir, edge) {
            diffuse_ray(grid, energy_map, Ray { row: next.0, col: next.1, dir: next_dir}, edge);
        }
    }
}

#[derive(Debug)]
struct Ray {
    row: usize,
    col: usize,
    dir: Dir,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    R,L,U,D
}

impl Dir {
    fn flip_rd(&self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
            Dir::D => Dir::R,
        }
    }
    fn flip_ru(&self) -> Dir {
        match self {
            Dir::R => Dir::U,
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::D,
        }
    }
    fn as_flag(&self) -> u8 {
        match self {
            Dir::R => R,
            Dir::U => U,
            Dir::D => D,
            Dir::L => L,
        }
    }
}

const R: u8 = 1;
const U: u8 = 2;
const D: u8 = 4;
const L: u8 = 8;
