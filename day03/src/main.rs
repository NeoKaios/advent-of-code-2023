use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");
    first_part(&content);
    second_part(&content);
}

fn second_part<'a>(content: &String) -> Option<&str> {
    let length = content.lines().next()?.len();
    let pad = ".".repeat(length+1);
    let txt = format!("{pad}\n{content}\n{pad}");

    let mut gears: HashMap<usize, Vec<i32>> = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();
    let mut offset =0;
    loop {
        let progress = match txt.get(offset..) {
            None => break,
            Some(t) => t,
        };
        let mat = match re.find(progress) {
            None => break,
            Some(t) => t,
        };
        let start = mat.start() + offset;
        let end = mat.end() + offset;

        let value = txt.get(start..end)?.parse().ok()?;
        // println!("{}", value);
        let mut keys = vec!();
        if let Some(before) = txt.get(start-1..start) {
            if  before.as_bytes()[0] == b'*' {
                keys.push(start-1)
            }
        }
        if let Some(after) = txt.get(end..end+1) {
            if  after.as_bytes()[0] == b'*' {
                keys.push(end)
            }
        }
        if let Some(above) = txt.get(start-length-2..end-length) {
            for (idx,byte) in above.bytes().enumerate() {
                if byte == b'*' {
                    keys.push(idx+start-length-2)
                }
            }
        }
        if let Some(below) = txt.get(start+length..end+length+2) {
            for (idx,byte) in below.bytes().enumerate() {
                if byte == b'*' {
                    keys.push(idx+start+length)
                }
            }
        }
        offset = end;
        for key in keys {
            match gears.get_mut(&key) {
                Some(entry) => entry.push(value),
                None => {gears.insert(key, vec!(value));},
            }
        }
    }
    let mut sum = 0;
    for val in gears.values() {
        if val.len() == 2 {
            sum += val[0]*val[1];
        }

    }
    println!("Sum of all gears ratio is: {sum}");
    None
}

fn first_part(content: &String) -> Option<&str> {
    let length = content.lines().next()?.len();
    let pad = ".".repeat(length+1);
    let txt = format!("{pad}\n{content}\n{pad}");

    let re = Regex::new(r"\d+").unwrap();
    let mut offset =0;
    let mut sum =0;
    loop {
        let progress = match txt.get(offset..) {
            None => break,
            Some(t) => t,
        };
        let mat = match re.find(progress) {
            None => break,
            Some(t) => t,
        };
        let start = mat.start() + offset;
        let end = mat.end() + offset;
        offset = end;

        let value: i32 = txt.get(start..end)?.parse().ok()?;
        // println!("{}", value);
        if let Some(before) = txt.get(start-1..start) {
            let ch = before.as_bytes()[0];
            if  ch != b'.' && ch != b'\n' {
                sum+=value;
                continue;
            }
        }
        if let Some(after) = txt.get(end..end+1) {
            let ch = after.as_bytes()[0];
            if  ch != b'.' && ch != b'\n' {
                sum+=value;
                continue;
            }
        }
        if let Some(above) = txt.get(start-length-2..end-length) {
            if above.chars().any(|c| c != '.' && c != '\n') {
                sum+=value;
                continue;
            }
        }
        if let Some(below) = txt.get(start+length..end+length+2) {
            if below.chars().any(|c| c != '.' && c != '\n') {
                sum+=value;
                continue;
            }
        }
    }
    println!("Sum of parts is: {sum}");
    None
}

