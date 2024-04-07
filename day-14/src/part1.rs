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

fn tilt_platform(mut platform: Vec<Vec<Thing>>) -> Vec<Vec<Thing>> {
    for i in 1..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j] == Thing::Sphere {
                let mut movable = true;
                let mut i_index = i;
                while movable {
                    if i_index == 0 {
                        movable = false;
                    } else if platform[i_index][j] == Thing::Sphere
                        && platform[i_index - 1][j] == Thing::Empty
                    {
                        platform[i_index - 1][j] = Thing::Sphere;
                        platform[i_index][j] = Thing::Empty;
                        i_index -= 1;
                    } else {
                        movable = false;
                    }
                }
            }
        }
    }
    platform
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
    let platform = parse_platform(input);
    let load = calculate_load(&tilt_platform(platform));
    load.to_string()
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
        assert_eq!("136", process(input))
    }

    #[test]
    fn test_parse_platform() {
        let input = "O..#
.O.#";
        let platform = parse_platform(input);
        assert_eq!(
            platform,
            vec![
                vec![Thing::Sphere, Thing::Empty, Thing::Empty, Thing::Cube],
                vec![Thing::Empty, Thing::Sphere, Thing::Empty, Thing::Cube]
            ]
        )
    }

    #[test]
    fn test_tilt_platform() {
        let input = ".
O
.
.
O
O
#
.
O";
        let platform = parse_platform(input);
        let tilt = tilt_platform(platform);
        println!("{:?}", tilt);
        assert_eq!(Thing::Sphere, tilt[0][0]);
        assert_eq!(Thing::Sphere, tilt[1][0]);
        assert_eq!(Thing::Sphere, tilt[2][0]);
        assert_eq!(Thing::Empty, tilt[3][0]);
        assert_eq!(Thing::Empty, tilt[4][0]);
    }
}
