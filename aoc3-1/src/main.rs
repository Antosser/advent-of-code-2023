use colored::*;

#[derive(PartialEq)]
enum Character {
    Nothing,
    Symbol,
    Digit(u32),
}

fn main() {
    let input = include_str!("../input.txt");
    let converted = input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Character::Nothing
                    } else if let Some(digit) = c.to_digit(10) {
                        Character::Digit(digit)
                    } else {
                        Character::Symbol
                    }
                })
                .collect::<Vec<Character>>()
        })
        .collect::<Vec<Vec<Character>>>();

    for i in converted.iter() {
        for j in i {
            print!(
                "{}",
                match j {
                    Character::Nothing => ".".to_owned(),
                    Character::Symbol => "S".to_owned(),
                    Character::Digit(d) => d.to_string(),
                }
            )
        }
        println!();
    }

    let mut sum = 0;
    for (ln, line) in converted.iter().enumerate() {
        let mut start_x = 0;
        let mut digits = Vec::new();
        for (col, c) in line.iter().chain([Character::Nothing].iter()).enumerate() {
            if let Character::Digit(d) = c {
                if digits.is_empty() {
                    start_x = col;
                }
                digits.push(d);
            } else {
                if digits.is_empty() {
                    continue;
                }
                let num = digits.iter().fold(0, |acc, new| acc * 10 + **new);
                println!("Found: {}", num.to_string().red());
                if ((start_x as i32 - 1).max(0) as usize..(col + 1)).any(|x| {
                    ((ln as i32 - 1).max(0) as usize..=(ln + 1)).any(|y| {
                        converted
                            .get(y)
                            .unwrap_or(&Vec::new())
                            .get(x)
                            .unwrap_or(&Character::Nothing)
                            == &Character::Symbol
                    })
                }) {
                    sum += num;
                    println!("Valid: {}", num.to_string().green());
                }
                digits.clear();
            }
        }
    }

    println!("{}", sum);
}
