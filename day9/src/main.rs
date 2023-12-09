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

fn predict_next_val(history: &mut Vec<i32>, len: usize)->i32 {
    if history.iter().enumerate().all(|(idx,val)| *val == 0 || idx >= len-1) {
        return 0;
    }
    let last = history[len-1];
    for idx in 0..len-1 {
        history[idx] = history[idx+1] - history[idx];
    }
    last + predict_next_val(history, len -1)
}

fn first_part(content: &String) {
    let lines = content.lines();
    let re = Regex::new(r"[-\d]+").unwrap();
    let total = lines.fold(0, |acc,line| {
        let mut history: Vec<i32> = re.find_iter(line).map(|s| s.as_str().parse().expect("Failed to parse")).collect();
        let len = history.len();
        let next_val = predict_next_val(&mut history, len);
        acc+next_val
    });
    println!("Sum of predicted values is: {total}");
}

fn predict_previous_val(history: &mut Vec<i32>, len: usize)->i32 {
    if history.iter().enumerate().all(|(idx,val)| *val == 0 || idx >= len-1) {
        return 0;
    }
    let first = history[0];
    for idx in 0..len-1 {
        history[idx] = history[idx+1] - history[idx];
    }
    first - predict_previous_val(history, len-1)
}

fn second_part(content: &String) {
    let lines = content.lines();
    let re = Regex::new(r"[-\d]+").unwrap();
    let total = lines.fold(0, |acc,line| {
        let mut history: Vec<i32> = re.find_iter(line).map(|s| s.as_str().parse().expect("Failed to parse")).collect();
        let len = history.len();
        let next_val = predict_previous_val(&mut history, len);
        acc+next_val
    });
    println!("Sum of previous values is: {total}");
}
