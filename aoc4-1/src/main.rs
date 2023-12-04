use regex::Regex;

fn main() {
    let number_regex = Regex::new(r" (?<num>\d+)").unwrap();

    let input = include_str!("../input.txt");

    let mut total_points = 0;
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

        println!("Winning: {:?}", winning);
        println!("Have: {:?}", having);

        let mut matches = 0;
        for have in having.iter() {
            if winning.contains(have) {
                matches += 1;
            }
        }

        println!("Won: {}", matches);
        let scratchcard_points = 2f32.powi(matches - 1).floor() as i32;
        println!("Points: {}", scratchcard_points);

        total_points += scratchcard_points;
    }

    println!("{}", total_points);
}
