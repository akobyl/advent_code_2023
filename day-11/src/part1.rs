use itertools::Itertools;

fn manhatten_distance(y1: i32, x1: i32, y2: i32, x2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn get_galaxies(universe: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    for (i, row) in universe.iter().enumerate() {
        row.iter().enumerate().for_each(|(j, &g)| {
            if g {
                galaxies.push((i, j));
            }
        });
    }

    galaxies
}

fn expand_universe(universe: &mut Vec<Vec<bool>>) {
    // Add blank rows
    let mut new_universe = Vec::new();
    let blank_row = vec![false; universe[0].len()];
    for row in universe.iter() {
        if row.iter().all(|&g| !g) {
            new_universe.push(blank_row.clone());
        }
        new_universe.push(row.clone());
    }
    *universe = new_universe;

    // Add blank columns
    let mut cols = Vec::new();

    for col in 0..universe[0].len() {
        let this_col: Vec<_> = universe
            .iter()
            .filter_map(|row| row.get(col))
            .copied()
            .collect();
        if this_col.iter().all(|&g| !g) {
            cols.push(col);
        }
    }
    cols.sort_by(|a, b| b.cmp(a));

    for row in universe.iter_mut() {
        for col in &cols {
            row.insert(*col, false);
        }
    }
}

pub fn process(input: &str) -> String {
    let mut universe: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    expand_universe(&mut universe);
    let galaxies = get_galaxies(&universe);
    let pairs = galaxies.into_iter().combinations(2).collect_vec();

    let sum = pairs.iter().fold(0, |acc, v| {
        let a = v.first().unwrap();
        let b = v.last().unwrap();
        acc + manhatten_distance(a.0 as i32, a.1 as i32, b.0 as i32, b.1 as i32)
    });

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhatten_distance() {
        assert_eq!(9, manhatten_distance(6, 1, 11, 5));
        assert_eq!(15, manhatten_distance(0, 4, 10, 9));
        assert_eq!(17, manhatten_distance(2, 0, 7, 12));
        assert_eq!(5, manhatten_distance(11, 0, 11, 5));
    }

    #[test]
    fn test_get_galaxies() {
        let universe = vec![vec![false, true, false], vec![false, true, true]];
        let galaxies = get_galaxies(&universe);
        assert!(galaxies.contains(&(0, 1)));
        assert!(galaxies.contains(&(1, 1)));
        assert!(galaxies.contains(&(1, 2)));
    }

    #[test]
    fn test_process() {
        let input = "...#......\n\
                     .......#..\n\
                     #.........\n\
                     ..........\n\
                     ......#...\n\
                     .#........\n\
                     .........#\n\
                     ..........\n\
                     .......#..\n\
                     #...#.....";
        assert_eq!("374", process(input))
    }
}
