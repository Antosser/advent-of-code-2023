use regex::Regex;

fn main() {
    let number_regex = Regex::new(r" (?<num>\d+)").unwrap();

    let input = include_str!("../input.txt");

    let mut scratchcard_matches = Vec::new();
    for line in input.trim().split('\n') {
        let mut sequences = line.split([':', '|']).skip(1);

        let winning = number_regex
            .captures_iter(sequences.next().unwrap())
            .map(|num| num["num"].to_string())
            .collect::<Vec<String>>();
        let having = number_regex
            .captures_iter(sequences.next().unwrap())
            .map(|num| num["num"].to_string())
            .collect::<Vec<String>>();

        let mut matches = 0;
        for have in having.iter() {
            if winning.contains(have) {
                matches += 1;
            }
        }

        scratchcard_matches.push(matches);
    }

    let mut scratchcard_amount = 0;

    fn solve_scratchcard(i: usize, scratchcards: &mut Vec<i32>, scratchcard_amount: &mut i32) {
        *scratchcard_amount += 1;
        for j in (i + 1)..(i + 1 + scratchcards[i] as usize) {
            solve_scratchcard(j, scratchcards, scratchcard_amount);
        }
    }

    for i in 0..scratchcard_matches.len() {
        solve_scratchcard(i, &mut scratchcard_matches, &mut scratchcard_amount);
    }

    println!("{}", scratchcard_amount);
}
