use std::fs;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let number_of_winning: i32 = match &args[2].parse() {
        Err(_) => panic!("2nd arg needs to be a number, '{}' given", &args[2]),
        Ok(n) => *n,
    };

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content, number_of_winning);
    second_part(&content, number_of_winning);
}

fn second_part(content: &String, nb_number_win: i32) {
    let line_count = content.lines().count();
    let lines = content.lines();

    let mut scratch = vec![1; line_count];
    let re = Regex::new(r"\d+").unwrap();
    for (nb,line) in lines.enumerate() {
        let mut winning_number = vec![];
        let mut number_of_win = 0;
        for (idx,value) in re.find_iter(line).map(|m| m.as_str().parse().unwrap_or(0)).enumerate() {
            if idx == 0 {continue;}
            if idx as i32 <= nb_number_win {
                winning_number.push(value);
                continue;
            }
            if winning_number.contains(&value) {
                number_of_win+=1;
            }
        }
        let mult = scratch[nb];
        for i in 1..number_of_win+1 {
            if let Some(value) = scratch.get_mut(i+nb) {
                *value += mult;
            }
        }
    }
    let total: i32 = scratch.iter().sum();
    println!("Total number of scratchcards: {total}");
}

fn first_part(content: &String, nb_number_win: i32) {
    let lines = content.lines();

    let re = Regex::new(r"\d+").unwrap();
    let total = lines.fold(0, |acc,line| {
        let mut winning_number = vec!();
        let mut card_point = 0;
        for (idx,value) in re.find_iter(line).map(|m| m.as_str().parse().unwrap_or(0)).enumerate() {
            if idx == 0 {continue;}
            if idx as i32 <= nb_number_win {
                winning_number.push(value);
                continue;
            }
            if winning_number.contains(&value) {
                if card_point == 0 { card_point = 1; }
                else {card_point*=2;}
            }
        }
        acc + card_point
    });
    println!("Total card points: {total}");
}

