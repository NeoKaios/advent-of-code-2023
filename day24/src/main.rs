use std::{fs, env};

use nalgebra::{Matrix2, Vector2};
use regex::Regex;

const _RES:i64 = 566373506408017;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    let condition = if file_path == "input.txt" {
        (LOWER_BOUND, UPPER_BOUND)
    } else {
        (LOWER_BOUND_EXEMPLE, UPPER_BOUND_EXEMPLE)
    };
    compute(&content, condition);
}

const LOWER_BOUND_EXEMPLE: f64 = 7.;
const UPPER_BOUND_EXEMPLE: f64 = 27.;
const LOWER_BOUND: f64 = 200000000000000.;
const UPPER_BOUND: f64 = 400000000000000.;

fn compute(content: &String, test_area: (f64,f64)) {
    let re_hailstone = Regex::new(r"(\d+), (\d+), (\d+) @ +(-?\d+), +(-?\d+), +(-?\d+)").unwrap();
    let mut hailstones = Vec::new();
    content.lines().for_each(|line| hailstones.push(Hailstone::parse(line, &re_hailstone)));
    // Part 1 ============================
    {
        let mut sum = 0;
        for (i,h1) in hailstones.iter().enumerate() {
            for h2 in &hailstones[i+1..] {
                if h1.xy_path_cross(h2, test_area) {sum+=1;}
            }
        }
        println!("The number of hailstones that would cross is: {sum}");
    }
    // Part 2 ============================
    for abs_vx in 1.. { 'next: for sign in [-1,1] {
        let vx = abs_vx*sign;
        let mut all_possible_x = hailstones[0].to_bounded_mod_set(vx);
        for h in &hailstones[1..] {
            let possible_x = h.to_bounded_mod_set(vx);
            let Some(joined_possible_x) = all_possible_x.join(&possible_x) else {continue 'next;};
            all_possible_x = joined_possible_x;
            if all_possible_x.min != all_possible_x.max { continue; }
            if let Some(res) = check_3d_impact(&hailstones, all_possible_x.min as f64, vx as f64) {
                println!("Result is: {}", res);
                return;
            }
            continue 'next;
        }
        // Found a non single possible set
        println!("Found {:?}", all_possible_x);
        let mut x = all_possible_x.min;
        loop {
            if let Some(res) = check_3d_impact(&hailstones, x as f64, vx as f64) {
                println!("Result is: {}", res);
                return;
            }
            x+=all_possible_x.modulus;
        }
    } }
}

impl BoundedModSet {
    fn join(&self, other: &BoundedModSet) -> Option<BoundedModSet> {
        //join modulus
        let modulus = lcm(self.modulus, other.modulus);
        let mut i = self.mod_value;
        let mut mod_value = -1;
        while i < modulus {
            if i % other.modulus == other.mod_value {
                mod_value = i;
            }
            i+=self.modulus;
        }
        if mod_value == -1 { return None; }
        //join & optimize bound
        let mut max = std::cmp::min(self.max, other.max);
        let mut min = std::cmp::max(self.min, other.min);
        if min > i64::MIN {
            min += (mod_value - min%modulus + modulus)%modulus;
        }
        if max < i64::MAX {
            max -= (max%modulus - mod_value + modulus)%modulus;
        }
        if max < min { return None; }
        Some(BoundedModSet { modulus, mod_value, max, min, })
    }

}

#[derive(Debug)]
struct BoundedModSet {
    modulus: i64,
    mod_value: i64,
    min: i64,
    max: i64,
}

fn get_pos_and_speed(h0: &Hailstone, h1: &Hailstone, t0: f64, t1: f64, pf: &str, vf: &str) -> (f64,f64) {
    let v = ((h0.get(pf)+h0.get(vf)*t0) - (h1.get(pf) + h1.get(vf)*t1))/(t0-t1);
    let p = (h0.get(pf)+h0.get(vf)*t0)-v*t0;
    (p,v)
}

fn check_3d_impact(hailstones: &Vec<Hailstone>, x: f64, vx: f64) -> Option<f64> {
    let times = hailstones.iter().map(|h| (x-h.x)/(h.vx-vx)).collect::<Vec<f64>>();
    let t0 = times[0];
    let t1 = times[1];
    let (y,vy) = get_pos_and_speed(&hailstones[0], &hailstones[1], t0, t1, "y", "vy");
    let (z,vz) = get_pos_and_speed(&hailstones[0], &hailstones[1], t0, t1, "z", "vz");
    if vy.fract().abs() < 10e-10 && vz.fract().abs() < 10e-10 &&
        hailstones.iter().zip(times.iter()).all(|(h,t)| h.y+h.vy*t == y+vy*t) &&
            hailstones.iter().zip(times.iter()).all(|(h,t)| h.z+h.vz*t == z+vz*t) {
                return Some(x+y+z);
            }
    return None;
}

impl Hailstone {
    fn get(&self, field: &str) -> f64 {
        match field {
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            "vx" => self.vx,
            "vy" => self.vy,
            "vz" => self.vz,
            _ => panic!(),
        }
    }
    fn parse(s: &str, re: &Regex) -> Hailstone {
        let [x,y,z,vx,vy,vz] = re.captures(s).expect("Hailstone").extract().1.map(|to_parse| to_parse.parse::<f64>().expect("Working"));
        Hailstone { x, y, z, vx, vy, vz }
    }

    fn to_bounded_mod_set(&self, vx: i64) -> BoundedModSet {
        let x = self.x as i64;
        let imodulus = self.vx as i64 - vx;
        let modulus = imodulus.abs();
        let direction = imodulus.signum();
        if modulus == 0 {
            BoundedModSet { modulus: 1, mod_value: 0, max: x, min: x, }
        } else {
            BoundedModSet {
                modulus,
                mod_value: x % modulus,
                max: if direction == -1 {x} else {i64::MAX},
                min: if direction ==  1 {x} else {i64::MIN},
            }

        }
    }

    fn xy_path_cross(&self, other: &Hailstone, test_area: (f64,f64)) -> bool {
        let a = Matrix2::new(self.vx, -other.vx, self.vy, -other.vy);
        match a.try_inverse() {
            Some(inv_a) => {
                let b = Vector2::new(other.x-self.x, other.y-self.y);
                let res = inv_a*b;
                let x = self.x+self.vx*res[0];
                let y = self.y+self.vy*res[0];
                let lower_bound = test_area.0;
                let upper_bound = test_area.1;
                return res[0] > 0. && res[1] > 0.
                    && lower_bound <= x && x <= upper_bound &&
                    lower_bound <= y && y <= upper_bound
            }
            None => return false
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

