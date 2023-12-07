use regex::Regex;

fn main() {
    let regexp = Regex::new(r" (\d+)").unwrap();

    let mut input = include_str!("../input.txt").lines();

    let times = regexp.captures_iter(input.next().unwrap()).map(|capture| {
        capture
            .get(0)
            .unwrap()
            .as_str()
            .trim()
            .parse::<u32>()
            .unwrap()
    });
    let distances = regexp.captures_iter(input.next().unwrap()).map(|capture| {
        capture
            .get(0)
            .unwrap()
            .as_str()
            .trim()
            .parse::<u32>()
            .unwrap()
    });

    let pairs = times.zip(distances);

    let product = pairs
        .map(|pair| {
            (0..pair.0)
                .map(|time| time * pair.0 - time * time > pair.1)
                .filter(|b| *b)
                .count()
        })
        .product::<usize>();

    println!("Product: {}", product);
}
