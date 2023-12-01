fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        input
            .trim()
            .split('\n')
            .map(|s| {
                let mut first = (9999, None);
                let mut last: (i32, Option<u32>) = (-1, None);

                let pairs = [
                    ("1", 1),
                    ("2", 2),
                    ("3", 3),
                    ("4", 4),
                    ("5", 5),
                    ("6", 6),
                    ("7", 7),
                    ("8", 8),
                    ("9", 9),
                    ("one", 1),
                    ("two", 2),
                    ("three", 3),
                    ("four", 4),
                    ("five", 5),
                    ("six", 6),
                    ("seven", 7),
                    ("eight", 8),
                    ("nine", 9),
                ];

                for pair in pairs {
                    if let Some(index) = s.find(pair.0) {
                        if index < first.0 {
                            first = (index, Some(pair.1));
                        }
                    }
                    if let Some(index) = s.rfind(pair.0) {
                        if index as i32 > last.0 {
                            last = (index as i32, Some(pair.1));
                        }
                    }
                }

                first.1.unwrap() * 10 + last.1.unwrap()
            })
            .sum::<u32>()
    );
}
