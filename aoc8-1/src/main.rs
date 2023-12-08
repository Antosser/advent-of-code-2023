use core::panic;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let regexp = Regex::new(r"(?<a>[A-Z]{3}) = \((?<b>[A-Z]{3}), (?<c>[A-Z]{3})\)").unwrap();

    let mut input = include_str!("../input.txt").lines();

    let mut directions = input.next().unwrap().chars().cycle();

    let mut mapping: HashMap<String, (String, String)> = HashMap::new();
    for line in input {
        let captures = match regexp.captures(line) {
            Some(c) => c,
            None => continue,
        };

        mapping.insert(
            captures["a"].to_string(),
            (captures["b"].to_string(), captures["c"].to_string()),
        );
    }

    let mut position = "AAA".to_string();
    for moves in 1.. {
        match directions.next().unwrap() {
            'L' => position = mapping.get(&position).unwrap().0.clone(),
            'R' => position = mapping.get(&position).unwrap().1.clone(),
            _ => panic!(),
        }

        if position == "ZZZ" {
            println!("Moves: {}", moves);
            break;
        }
    }
}
