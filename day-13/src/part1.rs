#[derive(Debug, PartialEq, Eq)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

fn rotate_matrix(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i]).collect())
        .collect()
}

//fn find_split(input: Vec<Vec<char>>) -> Option<usize> {
fn find_split(input: &[Vec<char>]) -> Option<usize> {
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            let mut match_found = true;

            // If the match is at the very beginning or end of the input
            if i == 0 {
                return Some(1);
            }
            if i == input.len() - 1 {
                return Some(input.len());
            }
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
                return Some(i + 1);
            }
        }
    }
    None
}

fn find_mirror(input: Vec<Vec<char>>) -> Mirror {
    if let Some(i) = find_split(&input) {
        return Mirror::Horizontal(i);
    }
    let rotated = rotate_matrix(input);
    if let Some(i) = find_split(&rotated) {
        return Mirror::Vertical(i);
    }
    assert!(false, "No mirror found");
    return Mirror::Vertical(0);
}

pub fn process(input: &str) -> String {
    let mut mirrors: Vec<Mirror> = vec![];
    let chunks: Vec<&str> = input.split("\n\n").collect();
    for chunk in chunks {
        let map: Vec<Vec<char>> = chunk.lines().map(|line| line.chars().collect()).collect();
        mirrors.push(find_mirror(map));
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
        assert_eq!("405", process(input))
    }

    #[test]
    fn find_horizontal() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(Some(4), find_split(&map));
    }

    #[test]
    fn find_vertical() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        assert_eq!(Mirror::Vertical(5), find_mirror(map));
    }

    #[test]
    fn test_rotate_1() {
        let map_vec = vec![vec!['1', '2', '2', '2', '3'], vec!['1', '2', '4', '4', '4']];
        let rotated = rotate_matrix(map_vec);
        assert_eq!(
            vec![
                vec!['1', '1'],
                vec!['2', '2'],
                vec!['2', '4'],
                vec!['2', '4'],
                vec!['3', '4']
            ],
            rotated
        );
    }

    #[test]
    fn test_some_eq() {
        let t1 = Some(vec!['1', '2']);
        let t2 = Some(vec!['1', '2']);
        assert_eq!(t1, t2);

        let t3 = Some(vec!['1', '3']);
        assert_ne!(t1, t3);
    }
}
