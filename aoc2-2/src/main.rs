use regex::Regex;

fn main() {
    let red: Regex = Regex::new(r"(?<num>\d+) red").unwrap();
    let green: Regex = Regex::new(r"(?<num>\d+) green").unwrap();
    let blue: Regex = Regex::new(r"(?<num>\d+) blue").unwrap();

    let input = include_str!("../input_test.txt");

    let mut sum = 0;

    for line in input.trim().split('\n') {
        let mut min = [0, 0, 0];

        for (j, color) in [&red, &green, &blue].iter().enumerate() {
            for num in color.captures_iter(line) {
                let num = num["num"].parse::<u32>().unwrap();

                min[j] = min[j].max(num);
            }
        }

        sum += min.iter().product::<u32>();
    }

    println!("{}", sum);
}
