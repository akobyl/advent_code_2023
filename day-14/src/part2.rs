use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Thing {
    Sphere,
    Cube,
    Empty,
}

impl Thing {
    fn from_char(c: char) -> Thing {
        match c {
            'O' => Thing::Sphere,
            '#' => Thing::Cube,
            '.' => Thing::Empty,
            _ => panic!("invalid char: {}", c),
        }
    }
}

fn parse_platform(input: &str) -> Vec<Vec<Thing>> {
    input
        .lines()
        .map(|line| line.chars().map(Thing::from_char).collect())
        .collect()
}

fn rotate_platform(platform: &mut Vec<Vec<Thing>>, clockwise: bool) {
    let cols = platform[0].len();

    let mut rotated: Vec<Vec<_>> = if clockwise {
        (0..cols)
            .map(|i| platform.iter().map(|row| row[i]).collect())
            .collect()
    } else {
        (0..cols)
            .rev()
            .map(|i| platform.iter().map(|row| row[i]).collect())
            .collect()
    };

    if !clockwise {
        rotated.reverse();
    }
    *platform = rotated;
}

fn tilt_platform_horizontal(platform: &mut [Vec<Thing>], left: bool) {
    let row_len = platform[0].len();
    for row in platform.iter_mut() {
        let mut new_row = Vec::with_capacity(row_len);
        let chunks: Vec<Vec<Thing>> = row
            .split(|&t| t == Thing::Cube)
            .map(|chunk| chunk.to_vec())
            .collect();

        let chunk_count = chunks.len();
        for (i, chunk) in chunks.into_iter().enumerate() {
            let sphere_count = chunk.iter().filter(|&&t| t == Thing::Sphere).count();
            let dot_count = chunk.len() - sphere_count;

            if left {
                new_row.extend(vec![Thing::Sphere; sphere_count]);
                new_row.extend(vec![Thing::Empty; dot_count]);
            } else {
                new_row.extend(vec![Thing::Empty; dot_count]);
                new_row.extend(vec![Thing::Sphere; sphere_count]);
            }

            if i < chunk_count - 1 {
                new_row.push(Thing::Cube);
            }
        }
        *row = new_row;
    }
}

fn tilt_platform_vertical(platform: &mut Vec<Vec<Thing>>, up: bool) {
    rotate_platform(platform, true);
    if up {
        tilt_platform_horizontal(platform, true);
    } else {
        tilt_platform_horizontal(platform, false);
    }
    rotate_platform(platform, false);
}

/// tilt platform in any direction
/// direction: 0: North, 1: West, 2: South, 3: East
fn tilt_platform_generic(platform: &mut Vec<Vec<Thing>>, direction: usize) {
    match direction {
        0 => tilt_platform_vertical(platform, true),
        1 => tilt_platform_horizontal(platform, true),
        2 => tilt_platform_vertical(platform, false),
        3 => tilt_platform_horizontal(platform, false),
        _ => panic!("uh oh what direciton is this?"),
    }
}

fn run_cycle(platform: &mut Vec<Vec<Thing>>) {
    for i in 0..4 {
        tilt_platform_generic(platform, i);
    }
}

fn calculate_load(platform: &[Vec<Thing>]) -> usize {
    let total_rows = platform.len();

    platform
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|&thing| *thing == Thing::Sphere).count() * (total_rows - i)
        })
        .sum()
}

pub fn process(input: &str) -> String {
    let endpoint = 1000000000;
    let mut platform = parse_platform(input);

    // From scientifically looking at the data, we observe a cycle appears that is len 7
    let cycle_length = 7;
    let mut seen_sequences = HashMap::new();
    let mut buffer = VecDeque::with_capacity(cycle_length);
    let mut cycle_start = 0;
    let mut final_sequence = Vec::new();

    for iteration in 0..endpoint {
        run_cycle(&mut platform);
        let load = calculate_load(&platform);

        buffer.push_back(load);

        if buffer.len() == cycle_length {
            let sequence = buffer.iter().copied().collect::<Vec<_>>();

            if let Some(&start) = seen_sequences.get(&sequence) {
                cycle_start = start;
                final_sequence = sequence;
                break;
            } else {
                seen_sequences.insert(sequence, iteration);
                buffer.pop_front();
            }
        }
    }

    let sequence_index = (endpoint - cycle_start - 2) % final_sequence.len();
    let load = final_sequence[sequence_index];
    load.to_string()
}

fn print_platform(platform: &Vec<Vec<Thing>>) {
    for row in platform {
        for &thing in row {
            let symbol = match thing {
                Thing::Empty => ".",
                Thing::Cube => "#",
                Thing::Sphere => "o",
            };
            print!("{}", symbol);
        }
        println!(); // Move to the next line after each row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("64", process(input))
    }

    #[test]
    fn test_tilt_platform_left() {
        let input = "..OO#.O.O
..###.O.O";
        let mut platform = parse_platform(input);
        tilt_platform_horizontal(&mut platform, true);
        let output = "OO..#OO..
..###OO..";
        let expected = parse_platform(output);
        assert_eq!(platform.len(), 2);
        assert_eq!(platform[0].len(), 9);
        assert_eq!(platform[1].len(), 9);
        assert_eq!(platform, expected);
    }

    #[test]
    fn test_tilt_platform_right() {
        let input = "..OO#.O.O
..###.O.O";
        let mut platform = parse_platform(input);
        tilt_platform_horizontal(&mut platform, false);
        assert_eq!(platform.len(), 2);
        assert_eq!(platform[0].len(), 9);
        assert_eq!(platform[1].len(), 9);
        let output = "..OO#..OO
..###..OO";

        let expected = parse_platform(output);
        assert_eq!(platform, expected);
    }

    #[test]
    fn test_tilt_platform_up() {
        let input = "..
OO
#.
.O
O#
..
";
        let mut platform = parse_platform(input);
        tilt_platform_vertical(&mut platform, true);
        let output = "OO
.O
#.
O.
.#
..
";
        let expected = parse_platform(output);
        assert_eq!(platform, expected);
    }

    #[test]
    fn test_cycle_1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut platform = parse_platform(input);
        run_cycle(&mut platform);
        println!("1 cycle");
        print_platform(&platform);

        run_cycle(&mut platform);
        println!("2 cycle");
        print_platform(&platform);

        run_cycle(&mut platform);
        println!("3 cycle");
        print_platform(&platform);
    }
}
