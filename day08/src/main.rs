use std::fs;
use std::collections::HashMap;
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

// From https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn second_part(content: &String) {
    let mut lines = content.lines();
    let re = Regex::new(r"[\d|A-Z]{3}").unwrap();
    let end_a = Regex::new(r"A$").unwrap();
    let end_z = Regex::new(r"Z$").unwrap();

    let instructions = lines.next().map_or("",|l| l);
    lines.next();

    let mut node_map: HashMap<&str, (&str,&str)> = HashMap::new();
    let mut starting_node = vec!();
    for map_line in lines {
        let map_entry: Vec<&str> = re.find_iter(map_line).map(|m| m.as_str()).collect();
        node_map.insert(map_entry[0], (map_entry[1], map_entry[2]));
        if end_a.is_match(map_entry[0]) {
            starting_node.push(map_entry[0]);
        }
    }
    let loops: Vec<u64> = starting_node.into_iter().map(|n| {
        let mut node = n;
        let mut nb_steps: u64 = 0;
        loop {
            for instr in instructions.chars() {
                nb_steps+=1;
                node = get_next_node(node, instr, &node_map);
                if end_z.is_match(node) {
                    return nb_steps;
                }
            }
        }
    }).collect();
    let lcm = lcm(&loops[..]);
    println!("Here are all the loops: {:?}", loops);
    println!("The number of steps for ghosts to reach __Z is: {lcm}");
}

fn get_next_node<'a>(node: &str, instr: char, node_map: &HashMap<&str, (&'a str,&'a str)>) -> &'a str {
    let lr_node = node_map.get(node).expect("Node not found");
    if instr == 'L' {
        lr_node.0
    } else {
        lr_node.1
    }
}

fn first_part(content: &String) {
    let mut lines = content.lines();
    let re = Regex::new(r"[A-Z]{3}").unwrap();
    let instructions = lines.next().map_or("",|l| l);
    let mut node_map: HashMap<&str, (&str,&str)> = HashMap::new();
    lines.next();
    for map_line in lines {
        let map_entry: Vec<&str> = re.find_iter(map_line).map(|m| m.as_str()).collect();
        node_map.insert(map_entry[0], (map_entry[1], map_entry[2]));
    }
    let mut nb_steps = 0;
    let mut node = "AAA";
    'outer: loop {
        for instr in instructions.chars() {
            nb_steps+=1;
            node = get_next_node(node, instr, &node_map);
            if node == "ZZZ" {
                break 'outer;
            }
        }
    }
    println!("The number of steps to reach ZZZ is: {nb_steps}");
}
