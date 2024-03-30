#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

fn rotate_matrix(input: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i]).collect())
        .collect()
}

fn find_splits(input: &[Vec<char>]) -> Vec<usize> {
    let mut splits: Vec<usize> = vec![];
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            let mut match_found = true;

            // If the match is at the very beginning or end of the input
            if i == 0 {
                splits.push(1);
            } else if i == input.len() - 1 {
                splits.push(input.len());
            } else {
                let mut i_top = i - 1;
                let mut i_bottom = i + 2;

                while match_found {
                    let upper = input.get(i_top);
                    let lower = input.get(i_bottom);

                    if (upper.is_some() && lower.is_some()) && (upper != lower) {
                        match_found = false;
                    }
                    if i_top == 0 || i_bottom == input.len() {
                        break;
                    }
                    i_top -= 1;
                    i_bottom += 1;
                }

                if match_found {
                    splits.push(i + 1);
                }
            }
        }
    }
    splits
}

fn find_mirrors(input: &[Vec<char>]) -> Vec<Mirror> {
    let mut mirrors: Vec<Mirror> = vec![];

    find_splits(input)
        .iter()
        .for_each(|val| mirrors.push(Mirror::Horizontal(*val)));

    let rotated = rotate_matrix(input);
    find_splits(&rotated)
        .iter()
        .for_each(|val| mirrors.push(Mirror::Vertical(*val)));

    mirrors
}

fn find_smudge_mirror(mut input: Vec<Vec<char>>) -> Mirror {
    let og_mirrors = find_mirrors(&input);
    if og_mirrors.len() > 1 {
        panic!("More than one mirror");
    }
    let og_mirror = og_mirrors.first().unwrap();

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let og_val = input[i][j];

            input[i][j] = match og_val {
                '.' => '#',
                '#' => '.',
                _ => panic!("invalid input"),
            };

            let splits = find_mirrors(&input);

            for mirror in &splits {
                if mirror != og_mirror {
                    return *mirror;
                }
            }
            input[i][j] = og_val;
        }
    }

    panic!("Could not find split");
}

pub fn process(input: &str) -> String {
    let mut mirrors: Vec<Mirror> = vec![];
    let chunks: Vec<&str> = input.split("\n\n").collect();
    for chunk in chunks {
        let map: Vec<Vec<char>> = chunk.lines().map(|line| line.chars().collect()).collect();
        mirrors.push(find_smudge_mirror(map));
    }
    let score = mirrors.iter().fold(0, |acc, m| match m {
        Mirror::Vertical(value) => acc + value,
        Mirror::Horizontal(value) => acc + (value * 100),
    });
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("400", process(input))
    }

    #[test]
    fn test_mirror_1() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(Mirror::Horizontal(3), find_smudge_mirror(map));
    }

    #[test]
    fn test_mirror_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(Mirror::Horizontal(1), find_smudge_mirror(map));
    }
}
