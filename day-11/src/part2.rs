use itertools::Itertools;

const EMPTY: u32 = 0;
const GALAXY: u32 = 1;
const ROW_GAP: u32 = 2;
const COL_GAP: u32 = 4;

fn manhatten_distance(
    universe: &[Vec<u32>],
    gap_size: i64,
    y1: i64,
    x1: i64,
    y2: i64,
    x2: i64,
) -> i64 {
    let x_0 = std::cmp::min(x1, x2) as usize;
    let x_1 = std::cmp::max(x1, x2) as usize;
    let y_0 = std::cmp::min(y1, y2) as usize;
    let y_1 = std::cmp::max(y1, y2) as usize;

    let row_gaps = universe
        .iter()
        .skip(y_0)
        .take(y_1 - y_0)
        .map(|row| row.first().unwrap())
        .filter(|&e| e & ROW_GAP != 0)
        .count() as i64;

    let col_gaps = universe
        .first()
        .unwrap()
        .iter()
        .skip(x_0)
        .take(x_1 - x_0)
        .filter(|&e| e & COL_GAP != 0)
        .count() as i64;

    (x1 - x2).abs() + (y1 - y2).abs() + row_gaps * (gap_size - 1) + col_gaps * (gap_size - 1)
}

fn get_galaxies(universe: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();

    for (i, row) in universe.iter().enumerate() {
        row.iter().enumerate().for_each(|(j, &g)| {
            if g == GALAXY {
                galaxies.push((i, j));
            }
        });
    }

    galaxies
}

fn expand_universe(universe: &mut Vec<Vec<u32>>) {
    // Add blank rows
    let mut new_universe = Vec::new();
    let row_gap = vec![ROW_GAP; universe[0].len()];
    for row in universe.iter() {
        if !row.iter().any(|&g| g == GALAXY) {
            new_universe.push(row_gap.clone());
        } else {
            new_universe.push(row.clone());
        }
    }

    // Add blank columns
    let mut cols = Vec::new();

    for col in 0..universe[0].len() {
        let this_col: Vec<_> = universe
            .iter()
            .filter_map(|row| row.get(col))
            .copied()
            .collect();
        if !this_col.iter().any(|&g| g == GALAXY) {
            cols.push(col);
        }
    }
    cols.sort_by(|a, b| b.cmp(a));

    for row in new_universe.iter_mut() {
        for col in &cols {
            row[*col] |= COL_GAP;
        }
    }
    *universe = new_universe;
}

pub fn process(input: &str, gap_size: i64) -> String {
    let mut universe: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => GALAXY,
                    _ => EMPTY,
                })
                .collect()
        })
        .collect();

    expand_universe(&mut universe);
    let galaxies = get_galaxies(&universe);
    let pairs = galaxies.into_iter().combinations(2).collect_vec();

    let sum: i64 = pairs.iter().fold(0, |acc, v| {
        let a = v.first().unwrap();
        let b = v.last().unwrap();
        acc + manhatten_distance(
            &universe, gap_size, a.0 as i64, a.1 as i64, b.0 as i64, b.1 as i64,
        )
    });

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_galaxies() {
        let universe = vec![vec![EMPTY, GALAXY, EMPTY], vec![EMPTY, GALAXY, GALAXY]];
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
        assert_eq!("8410", process(input, 100));
        assert_eq!("1030", process(input, 10));
    }
}
