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

fn predict_next_val(history: &Vec<i32>)->i32 {
    if history.iter().all(|val| *val == 0) {
        return 0;
    }
    let mut deriv_seq = vec![0; history.len()-1];
    for (idx, val) in deriv_seq.iter_mut().enumerate() {
        *val = history[idx+1] - history[idx];
    }
    history.last().unwrap_or(&0) + predict_next_val(&deriv_seq)
}

fn first_part(content: &String) {
    let lines = content.lines();
    let re = Regex::new(r"[-\d]+").unwrap();
    let total = lines.fold(0, |acc,line| {
        let history: Vec<i32> = re.find_iter(line).map(|s| s.as_str().parse().expect("Failed to parse")).collect();
        let next_val = predict_next_val(&history);
        acc+next_val
    });
    println!("Sum of predicted values is: {total}");
}

fn predict_previous_val(history: &Vec<i32>)->i32 {
    if history.iter().all(|val| *val == 0) {
        return 0;
    }
    let mut deriv_seq = vec![0; history.len()-1];
    for (idx, val) in deriv_seq.iter_mut().enumerate() {
        *val = history[idx+1] - history[idx];
    }
    history.first().unwrap_or(&0) - predict_previous_val(&deriv_seq)
}

fn second_part(content: &String) {
    let lines = content.lines();
    let re = Regex::new(r"[-\d]+").unwrap();
    let total = lines.fold(0, |acc,line| {
        let history: Vec<i32> = re.find_iter(line).map(|s| s.as_str().parse().expect("Failed to parse")).collect();
        let next_val = predict_previous_val(&history);
        acc+next_val
    });
    println!("Sum of previous values is: {total}");
}
