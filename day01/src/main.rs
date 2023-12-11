use std::fs;
use regex::Regex;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

fn second_part(content: &String) {
    let lines = content.lines();
    let regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    // Needed that because of "twone" = 21
    let xeger = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let total = lines.fold(0, |acc,line| {
        let enil: String = line.chars().rev().collect();
        let mut matches = regex.find_iter(line).map(|m| m.as_str());
        let mut sehctam = xeger.find_iter(&enil).map(|m| m.as_str());
        let first = match matches.next() {
            None => panic!("Found a line with no digits : \"{line}\""),
            Some(str) => cast_to_int(str),
        };
        let last = match sehctam.next() {
            None => panic!("Found a line with no digits : \"{line}\""),
            Some(str) => cast_to_int(str),
        };
        first*10 + last + acc
    });
    println!("Total with second method is : {total}");
}

fn cast_to_int(string: &str) -> u32 {
    match string {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => string.parse().map_or_else(|e| panic!("got unknown string {e}"), |i| i),
    }
}

fn first_part(content: &String) {
    let lines = content.lines();
    let total = lines.fold(0, |acc,line| {
        let chars = line.chars();
        let mut ch = chars.filter(|c| "123456789".contains(*c));
        let first = ch.next().map_or_else(|| panic!("No digits : '{line}'"), |char| char as u32 - 48);
        let last = ch.last().map_or(first, |char| char as u32 - 48);
        first*10 + last + acc
    });
    println!("Total with first method : {total}");
}
