use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    first_part(&content);
    // _first_part_opt(&content);
    second_part(&content);
}

fn second_part(content: &String) {
    let lines = content.lines();

    let total = lines.fold(Ok(0), |acc: Result<i32, &str>,line| {
        let acc = acc?;
        let parts = line.split(": ");
        let sets = parts.last().ok_or("No sets values")?.split("; ");
        let mut red_min = 0; let mut blue_min = 0; let mut green_min = 0;

        for set in sets {
            let dices = set.split(", ");
            for dice in dices {
                let mut num_color = dice.split(' ');
                let dice_amount = num_color.next().ok_or("No dice value")?.parse::<i32>().map_err(|_e| "parse err")?;
                let dice_color = num_color.next().ok_or("No dice color")?;
                let min = match dice_color {
                    "red" => &mut red_min,
                    "blue" => &mut blue_min,
                    "green" => &mut green_min,
                    _ => Err("Unknown color {dice_color}")?,
                };
                if *min < dice_amount {
                    *min = dice_amount;
                }
            }
        };
        Ok(red_min*blue_min*green_min + acc)
    });
    match total {
        Ok(tot) =>println!("Sum of power of games: {tot}"),
        Err(e) => println!("Some error: {:?}", e),
    }
}

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;
fn first_part(content: &String) {
    let lines = content.lines();

    let total = lines.fold(Ok(0), |acc: Result<i32, &str>,line| {
        let acc = acc?;
        let mut parts = line.split(": ");
        let game_id: i32 = match parts.next() {
            None => panic!("Weird line :{line}"),
            Some(str) => str.split(' ').last().ok_or("No game_id")?.parse().map_err(|_|"Parse error")?,
        };
        let sets = parts.next().ok_or("No sets")?.split("; ");
        for set in sets {
            let dices = set.split(", ");
            for dice in dices {
                let mut num_color = dice.split(' ');
                let dice_amount: i32 = num_color.next().ok_or("No dice value")?.parse().map_err(|_e| "parse err")?;
                let dice_color = num_color.next().ok_or("No dice color")?;
                let impossible = dice_amount > match dice_color {
                    "red" => RED_MAX,
                    "blue" => BLUE_MAX,
                    "green" => GREEN_MAX,
                    _ => Err("Unknown color {dice_color}")?,
                };
                if impossible {
                    return Ok(acc);
                }
            }
        };
        Ok(game_id + acc)
    });
    match total {
        Ok(tot) =>println!("Sum of possible game is: {tot}"),
        Err(e) => println!("Some error: {:?}", e),
    }
}

fn _first_part_opt(content: &String) {
    let lines = content.lines();

    let total = lines.fold(Some(0), |acc: Option<i32>,line| {
        let acc = acc?;
        let mut parts = line.split(": ");
        let game_id: i32 = match parts.next() {
            None => panic!("Weird line :{line}"),
            Some(str) => str.split(' ').last()?.parse().ok()?,
        };
        let sets = parts.next()?.split("; ");
        for set in sets {
            let dices = set.split(", ");
            for dice in dices {
                let mut num_color = dice.split(' ');
                let dice_amount: i32 = num_color.next()?.parse().ok()?;
                let dice_color = num_color.next()?;
                let impossible = dice_amount > match dice_color {
                    "red" => RED_MAX,
                    "blue" => BLUE_MAX,
                    "green" => GREEN_MAX,
                    _ => return None,
                };
                if impossible {
                    return Some(acc);
                }
            }
        };
        Some(game_id + acc)
    });
    match total {
        Some(tot) =>println!("Sum of possible game is: {tot}"),
        _ => panic!("Some error occured")
    }
}
