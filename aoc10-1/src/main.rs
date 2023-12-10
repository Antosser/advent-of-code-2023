use std::collections::HashMap;

use itertools::Itertools;

const UP: [i32; 2] = [0, 1];
const DOWN: [i32; 2] = [0, -1];
const RIGHT: [i32; 2] = [1, 0];
const LEFT: [i32; 2] = [-1, 0];

fn add(a: [i32; 2], b: [i32; 2]) -> [i32; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

fn main() {
    let input = include_str!("../test.txt");
    let start_tile = [DOWN, RIGHT];
    let mut start_tile_pos = None;

    let board = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_tile_pos = Some([x, y]);
                    }
                    match c {
                        '|' => [UP, DOWN],
                        '-' => [RIGHT, LEFT],
                        'L' => [UP, RIGHT],
                        'J' => [UP, LEFT],
                        '7' => [LEFT, DOWN],
                        'F' => [RIGHT, DOWN],
                        _ => start_tile,
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let start_tile_pos = start_tile_pos.unwrap();
    let mut distance_from_tiles: HashMap<(i32, i32), i32> = HashMap::new();

    for direction in board[start_tile_pos[1]][start_tile_pos[0]].iter() {
        let mut pos = add(start_tile_pos as [i32; 2], direction);
    }
}
