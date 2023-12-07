#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut conversions: Vec<Vec<Mapping>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            conversions.push(Vec::new());
            continue;
        }

        let numbers_on_line = line
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        conversions.last_mut().unwrap().push(Mapping {
            destination: numbers_on_line[0],
            source: numbers_on_line[1],
            range: numbers_on_line[2],
        });
    }

    let lowest_location = seeds
        .iter()
        .map(|seed| {
            let mut thing = *seed;

            'conversion: for conversion in conversions.iter() {
                for mapping in conversion {
                    if (mapping.source..(mapping.source + mapping.range)).contains(&thing) {
                        thing += mapping.destination - mapping.source;
                        continue 'conversion;
                    }
                }
            }

            thing
        })
        .min()
        .unwrap();

    println!("Lowest location: {}", lowest_location);
}
