fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        input
            .trim()
            .split('\n')
            .map(|s| {
                let mut first = None;
                let mut last = None;

                for c in s.chars() {
                    match c.to_digit(10) {
                        None => {}
                        Some(n) => {
                            first = Some(first.unwrap_or(n));
                            last = Some(n);
                        }
                    }
                }

                first.unwrap() * 10 + last.unwrap()
            })
            .sum::<u32>()
    );
}
