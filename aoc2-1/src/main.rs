use regex::Regex;

static MAX_CUBES: [u32; 3] = [12, 13, 14];

fn main() {
    let red: Regex = Regex::new(r"(?<num>\d+) red").unwrap();
    let green: Regex = Regex::new(r"(?<num>\d+) green").unwrap();
    let blue: Regex = Regex::new(r"(?<num>\d+) blue").unwrap();

    let input = include_str!("../input.txt");

    let mut sum = 0;

    'line: for (i, line) in input.trim().split('\n').enumerate() {
        for (j, color) in [&red, &green, &blue].iter().enumerate() {
            for num in color.captures_iter(line) {
                let num = num["num"].parse::<u32>().unwrap();

                if num > MAX_CUBES[j % 3] {
                    continue 'line;
                }
            }
        }

        sum += i + 1;
    }

    println!("{}", sum);
}
