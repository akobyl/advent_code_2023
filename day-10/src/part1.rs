use std::collections::HashMap;
use std::collections::VecDeque;

fn find_start(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == 'S') {
            return Some((i, j));
        }
    }
    None
}

fn get_touching_pipes(map: &[Vec<char>], y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut smap: HashMap<(i32, i32), &str> = HashMap::new();
    smap.insert((-1, 0), "|7F"); // Up one
    smap.insert((0, 1), "-7J"); // Right one
    smap.insert((0, -1), "-FL"); // Left one
    smap.insert((1, 0), "|LJ"); // Down one

    let x_0 = x as i32;
    let y_0 = y as i32;
    let x_max = map[0].len() as i32;
    let y_max = map.len() as i32;

    smap.iter()
        .filter_map(|(&k, &v)| {
            let (yd, xd) = (y_0 + k.0, x_0 + k.1);
            if yd >= 0
                && yd < y_max
                && xd >= 0
                && xd < x_max
                && v.contains(map[yd as usize][xd as usize])
            {
                Some((yd as usize, xd as usize))
            } else {
                None
            }
        })
        .collect()
}

pub fn process(input: &str) -> String {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&map).unwrap();
    let mut dists = vec![vec![None; map[0].len()]; map.len()];

    let mut stack: VecDeque<(usize, usize, i32)> = VecDeque::new();
    stack.push_back((start.0, start.1, 0));
    let mut peak_distance = 0;

    while let Some((y, x, dist)) = stack.pop_front() {
        if dists[y][x].is_some() {
            continue;
        }
        if dist > peak_distance {
            peak_distance = dist;
        }

        dists[y][x] = Some(dist);
        let next_points = get_touching_pipes(&map, y, x);
        // println!("({}, {}) -> {:?}", y, x, next_points);
        next_points
            .iter()
            .for_each(|&p| stack.push_back((p.0, p.1, dist + 1)));
    }
    peak_distance.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_touching_pipes() {
        let input = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '7', 'x', 'x', 'x'],
            vec!['|', 'S', '-', 'x', 'x'],
            vec!['.', 'F', '.', '.', '.'],
        ];
        let output = get_touching_pipes(&input, 2, 1);
        assert!(output.contains(&(1, 1)));
        assert!(output.contains(&(2, 2)));

        let output = get_touching_pipes(&input, 1, 0);
        assert!(output.contains(&(2, 0)));
    }

    #[test]
    fn test_find_start() {
        let input = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '7', 'x', 'x', 'x'],
            vec!['.', 'S', 'x', 'x', 'x'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert_eq!(Some((2, 1)), find_start(&input));

        let input = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '7', 'x', 'x', 'x'],
            vec!['.', 'x', 'x', 'x', 'x'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert!(find_start(&input).is_none());
    }

    #[test]
    fn test_process() {
        let input = ".....\n\
                     .S-7.\n\
                     .|.|.\n\
                     .L-J.\n\
                     .....";
        assert_eq!("4", process(input))
    }
}
