use std::ops::IndexMut;

use itertools::Itertools;
use regex::Regex;

static CARDS: &str = "23456789TJQKA";

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    score: u32,
}

fn score(cards: &str) -> u32 {
    if cards.chars().tuples().all(|(a, b)| a == b) {
        return 7;
    }

    let mut sorted_instances = vec![0; 13];
    for card in cards.chars() {
        *sorted_instances.index_mut(CARDS.find(card).unwrap()) += 1;
    }
    sorted_instances.sort();
    sorted_instances.reverse();

    if sorted_instances[0] == 4 {
        return 6;
    }
    if sorted_instances[0] == 3 && sorted_instances[1] == 2 {
        return 5;
    }
    if sorted_instances[0] == 3 && sorted_instances[1] == 1 && sorted_instances[2] == 1 {
        return 4;
    }
    if sorted_instances[0] == 2 && sorted_instances[1] == 2 {
        return 3;
    }
    if sorted_instances[0] == 2 && sorted_instances[1] == 1 && sorted_instances[2] == 1 {
        return 2;
    }

    1
}

fn main() {
    let line_regexp = Regex::new(r"(?<cards>[AKQJT98765432]{5}) (?<bid>\d+)$").unwrap();

    let input = include_str!("../input.txt");

    let mut hands = Vec::new();
    for line in input.lines() {
        let captures = line_regexp.captures(line).unwrap();

        let cards = captures["cards"].to_string();
        hands.push(Hand {
            score: score(&cards),
            cards,
            bid: captures["bid"].parse::<u32>().unwrap(),
        });
    }

    hands.sort_by(|a, b| {
        if a.score != b.score {
            return a.score.cmp(&b.score);
        }

        for i in 0..5 {
            if a.cards.as_bytes()[i] != b.cards.as_bytes()[i] {
                return CARDS
                    .find(a.cards.as_bytes()[i] as char)
                    .unwrap()
                    .cmp(&CARDS.find(b.cards.as_bytes()[i] as char).unwrap());
            }
        }

        panic!()
    });

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum();

    println!("Winnings: {}", winnings);
}
