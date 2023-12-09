use itertools::Itertools;

fn derivative(list: &[i64]) -> Vec<i64> {
    list.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn main() {
    let input = include_str!("../input.txt");

    let sum = input
        .trim()
        .lines()
        .map(|line| {
            let mut numbers = vec![line
                .trim()
                .split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()];

            while !numbers.last().unwrap().iter().all(|item| *item == 0) {
                numbers.push(derivative(numbers.last().unwrap()));
            }

            numbers
                .iter()
                .map(|list| list.first().unwrap())
                .enumerate()
                .fold(
                    0,
                    |acc, (i, item)| if i % 2 == 0 { acc + item } else { acc - item },
                )
        })
        .sum::<i64>();

    println!("Sum: {}", sum);
}
