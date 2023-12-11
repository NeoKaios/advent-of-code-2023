use std::fs;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

fn second_part(content: &String) {
    let mut lines = content.lines();
    let re = Regex::new(r"\d+").unwrap();
    let time_str = lines.next().map_or("",|l| l).replace(" ", "");
    let time: i64 = re.find(&time_str).map_or(0, |m| m.as_str().parse().unwrap_or(0));
    let dist_str = lines.next().map_or("",|l| l).replace(" ", "");
    let distance: i64 = re.find(&dist_str).map_or(0, |m| m.as_str().parse().unwrap_or(0));

    println!("The number of ways to win for p2: {}", solver(time, distance));
}

fn solver(time: i64, distance: i64) -> i64 {
    let delta: f64 = (time*time - 4*(-1)*-distance) as f64;
    let x1 = (-(time as f64) + delta.sqrt())/ (-2 as f64);
    let x2 = (-(time as f64) - delta.sqrt())/ (-2 as f64);
    (x2.floor() - x1.ceil() + 1.0) as i64
}

fn _bruteforcer(time: i64, distance: i64) -> i64 {
    let mut acc = 0;
    for x in 0..time+1 {
        if x*(time-x) > distance {
            acc+=1;
        }
    }
    acc
}

fn first_part(content: &String) {
    let mut lines = content.lines();
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<i32> = re.find_iter(lines.next().map_or("",|l| l)).map(|m| m.as_str().parse().unwrap_or(0)).collect();
    let distances: Vec<i32> = re.find_iter(lines.next().map_or("",|l| l)).map(|m| m.as_str().parse().unwrap_or(0)).collect();

    let mut mult = 1;
    for (idx,time) in times.iter().enumerate() {
        mult*=solver(*time as i64, distances[idx] as i64);
    }
    println!("The number of ways to win for p1: {mult}");
}
