use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
    direction: Direction,
}

fn is_next_in_grid(point: (i32, i32), direction: Direction, grid_size: (usize, usize)) -> bool {
    match direction {
        Direction::Up => point.1 > 0,
        Direction::Down => point.1 < grid_size.0 as i32 - 1,
        Direction::Left => point.0 > 0,
        Direction::Right => point.0 < grid_size.1 as i32 - 1,
    }
}

fn get_point_count(tiles: HashSet<Tile>) -> usize {
    tiles
        .iter()
        .map(|tile| (tile.x, tile.y))
        .fold(HashSet::new(), |mut acc, point| {
            acc.insert(point);
            acc
        })
        .len()
}

fn calculate_beam(
    grid: &[Vec<char>],
    grid_size: (usize, usize),
    start_x: i32,
    start_y: i32,
    start_dir: Direction,
) -> HashSet<Tile> {
    let mut seen_points = HashSet::new();

    let mut photons = VecDeque::new();
    photons.push_back(Tile {
        x: start_x,
        y: start_y,
        direction: start_dir,
    });

    while !photons.is_empty() {
        let photon = photons.pop_front().unwrap();

        if seen_points.contains(&photon) {
            continue;
        }

        seen_points.insert(photon);

        if !is_next_in_grid((photon.x, photon.y), photon.direction, grid_size) {
            continue;
        }

        let next_point = match photon.direction {
            Direction::Up => (photon.x, photon.y - 1),
            Direction::Down => (photon.x, photon.y + 1),
            Direction::Left => (photon.x - 1, photon.y),
            Direction::Right => (photon.x + 1, photon.y),
        };

        let x = next_point.0;
        let y = next_point.1;

        match grid[y as usize][x as usize] {
            '-' => match photon.direction {
                Direction::Down | Direction::Up => {
                    photons.push_back(Tile {
                        x,
                        y,
                        direction: Direction::Left,
                    });
                    photons.push_back(Tile {
                        x,
                        y,
                        direction: Direction::Right,
                    });
                }
                _ => photons.push_back(Tile {
                    x,
                    y,
                    direction: photon.direction,
                }),
            },
            '|' => match photon.direction {
                Direction::Left | Direction::Right => {
                    photons.push_back(Tile {
                        x,
                        y,
                        direction: Direction::Up,
                    });
                    photons.push_back(Tile {
                        x,
                        y,
                        direction: Direction::Down,
                    });
                }
                _ => {
                    photons.push_back(Tile {
                        x,
                        y,
                        direction: photon.direction,
                    });
                }
            },
            '\\' => {
                let direction = match photon.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                photons.push_back(Tile { x, y, direction });
            }
            '/' => {
                let direction = match photon.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                photons.push_back(Tile { x, y, direction });
            }
            _ => {
                photons.push_back(Tile {
                    x,
                    y,
                    direction: photon.direction,
                });
            }
        };
    }

    seen_points
}

pub fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_size = (grid.len(), grid[0].len());
    let mut max_sum = 0;

    // Left side
    for i in 0..grid_size.1 {
        let points = calculate_beam(&grid, grid_size, -1, i as i32, Direction::Right);
        let sum = get_point_count(points) - 1;
        if sum > max_sum {
            max_sum = sum;
        }

        let points = calculate_beam(
            &grid,
            grid_size,
            grid_size.0 as i32,
            i as i32,
            Direction::Left,
        );
        let sum = get_point_count(points) - 1;
        if sum > max_sum {
            max_sum = sum;
        }
    }

    for i in 0..grid_size.0 {
        let points = calculate_beam(&grid, grid_size, i as i32, -1, Direction::Down);
        let sum = get_point_count(points) - 1;
        if sum > max_sum {
            max_sum = sum;
        }

        let points = calculate_beam(
            &grid,
            grid_size,
            i as i32,
            grid_size.1 as i32,
            Direction::Up,
        );
        let sum = get_point_count(points) - 1;
        if sum > max_sum {
            max_sum = sum;
        }
    }

    max_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!("51", process(input))
    }
}
