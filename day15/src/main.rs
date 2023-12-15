use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

fn first_part(content: &String) {
    let len = content.len();
    let sum = content[..len-1].split(',').fold(0, |acc,s| acc + hash(s));
    println!("The sum of hash is: {sum}");
}

fn second_part(content: &String) {
    let mut boxes: Vec<Vec<Lens>> = vec![vec!(); 256];
    let len = content.len();
    for s in content[..len-1].split(',') {
        let step = step_maker(s);
        let h = hash(step.label);
        let box_ = &mut boxes[h as usize];
        match step.operation {
            Operation::Add(focus) => {
                match box_.iter_mut().find(|l| l.label == step.label) {
                    Some(l) => l.focus = focus,
                    None => box_.push(Lens {label: step.label, focus}),
                };
            }
            Operation::Remove => box_.retain(|l| l.label != step.label)
        }
    }
    let sum = boxes.iter().enumerate().fold(0, |total,(i,box_)| {
        let box_sum = box_.iter().enumerate().fold(0, |acc,(slot,lens)| {
            acc + (slot+1) as i32 * lens.focus as i32
        });
        total + box_sum*(i+1) as i32
    });
    println!("The focusing power is: {sum}");
}

fn hash(s: &str) -> i32 {
    s.as_bytes().iter().fold(0, |acc, byte| {
        ((acc+(*byte as i32))*17)%256
    })
}

fn step_maker(s: &str) -> Step {
    if s.contains('=') {
        let focus = s[s.len()-1..].parse().unwrap_or(0);
        Step { label: &s[..s.len()-2], operation: Operation::Add(focus)}
    }
    else {
        Step { label: &s[..s.len()-1], operation: Operation::Remove}
    }
}

#[derive(Clone, Copy, Debug)]
struct Lens<'a> {
    label: &'a str,
    focus: u8,
}

// #[derive(Debug)]
struct Step<'a> {
    label: &'a str,
    operation: Operation,
}

// #[derive(Debug)]
enum Operation {
    Add(u8),
    Remove,
}
