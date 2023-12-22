use std::{fs, env, collections::HashSet};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

fn compute(content: &String) {
    let lines = content.lines();
    let mut bricks: Vec<Brick> = vec!();

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let re = Regex::new(r"^(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)$").unwrap();

    for (i,line) in lines.enumerate() {
        let b = Brick::parse(line, i+1, &re);
        x = std::cmp::max(x, b.x2);
        y = std::cmp::max(y, b.y2);
        z = std::cmp::max(z, b.z2);
        bricks.push(b);
    }
    println!("{x} {y} {z}");
    let x = (x+1) as usize;
    let y = (y+1) as usize;
    let z = (z+1) as usize;
    let mut grid: Vec<Vec<Vec<u16>>> = vec![vec![vec![0;y]; x]; z];

    // ==== Fill grid with bricks
    for b in &bricks {
        b.fill(&mut grid);
    }

    // ==== Make gravity
    loop {
        let mut updated = false;
        for b in &mut bricks {
            while !b.is_supported(&grid) {
                b.drop(&mut grid);
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }

    // ==== Part 1 ============
    let sum = bricks.iter().fold(0, |acc,b| {
        let v = b.get_supported_bricks(&grid);
        for idx in v {
            if bricks[idx].get_supporting_bricks(&grid).len() == 1 {
                return acc;
            }
        }
        acc+1
    });
    println!("The number of desintegrable bricks is: {sum}");

    // ==== Part 2 ============
    let sum = bricks.iter().fold(0, |acc,b| {
        let mut would_fall = vec![(b.id-1) as usize; 1];
        let mut i = 0;
        loop {
            if i >= would_fall.len() { break; }
            for idx in bricks[would_fall[i]].get_supported_bricks(&grid) {
                if would_fall.contains(&idx) {
                    continue;
                }
                if bricks[idx].get_supporting_bricks(&grid).iter().all(|id| would_fall.contains(id)) {
                    would_fall.push(idx);
                }
            }
            i+=1;
        }
        acc + would_fall.len()-1
    });
    println!("The number of other brick that would fall is: {sum}");
}

#[derive(Debug)]
struct Brick {
    id: u16,

    x1: u16,
    y1: u16,
    z1: u16,
    x2: u16,
    y2: u16,
    z2: u16,
}

impl Brick {
    fn get_supported_bricks(&self, grid: &Vec<Vec<Vec<u16>>>) -> HashSet<usize> {
        let mut out = HashSet::new();
        for x in self.x1..self.x2+1 {
            for y in self.y1..self.y2+1 {
                if grid[(self.z2+1) as usize][x as usize][y as usize] != 0 {
                    out.insert((grid[(self.z2+1) as usize][x as usize][y as usize]-1) as usize);
                }
            }
        }
        out
    }
    fn get_supporting_bricks(&self, grid: &Vec<Vec<Vec<u16>>>) -> HashSet<usize> {
        let mut out = HashSet::new();
        for x in self.x1..self.x2+1 {
            for y in self.y1..self.y2+1 {
                if grid[(self.z1-1) as usize][x as usize][y as usize] != 0 {
                    out.insert((grid[(self.z1-1) as usize][x as usize][y as usize]-1) as usize);
                }
            }
        }
        out
    }
    fn is_supported(&self, grid: &Vec<Vec<Vec<u16>>>) -> bool {
        return self.z1 == 1 || !self.get_supporting_bricks(grid).is_empty()
    }

    fn fill(&self, grid: &mut Vec<Vec<Vec<u16>>>) {
        for z in self.z1..self.z2+1 {
            for x in self.x1..self.x2+1 {
                for y in self.y1..self.y2+1 {
                    grid[z as usize][x as usize][y as usize] = self.id;
                }
            }
        }
    }
    fn drop(&mut self, grid: &mut Vec<Vec<Vec<u16>>>) {
        self.z1 -=1;
        for x in self.x1..self.x2+1 {
            for y in self.y1..self.y2+1 {
                grid[self.z2 as usize][x as usize][y as usize] = 0;
                grid[self.z1 as usize][x as usize][y as usize] = self.id;
            }
        }
        self.z2 -=1;
    }

    fn parse(s: &str, i: usize, re: &Regex) -> Brick {
        let [x1,y1,z1, x2, y2, z2] = re.captures(s).expect("Should be brick coord").extract().1.map(|d| d.parse().expect("Digits"));
        Brick { x1, y1, z1, x2, y2, z2, id: i as u16 }
    }
}

fn _print_grid(grid: &Vec<Vec<u16>>) {
    for l in grid {
        for ch in l {
            print!("{ch} ");
        }
        print!("\n");
    }
}
