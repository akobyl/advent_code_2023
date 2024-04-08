struct Number {
    number: u32,
    x: i32,
    y: i32,
    tagged: bool,
}

fn get_surrounding_squares_tag(grid: &[Vec<char>], number: &mut Number) {
    let size = number.number.to_string().len() as i32;
    let symbols = ['*', '-', '+', '#', '&', '$', '=', '%', '@', '/'];

    for i in number.y - 1..=number.y + 1 {
        if i < 0 || i >= grid.len() as i32 {
            continue;
        }

        for j in number.x - 1..=number.x + size {
            if j < 0 || j >= grid[0].len() as i32 {
                continue;
            }
            if symbols.contains(&grid[i as usize][j as usize]) {
                number.tagged = true;
                return;
            }
        }
    }
}

pub fn process(_input: &str) -> String {
    let grid: Vec<Vec<char>> = _input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut numbers: Vec<Number> = Vec::new();

    // Process grid and mark numbers as visited
    for (i, row) in grid.iter().enumerate() {
        for (j, &pixel) in row.iter().enumerate() {
            if !visited[i][j] && pixel.is_ascii_digit() {
                // Found the start of a new number
                let mut num = Vec::new();
                for jnum in j..grid[0].len() {
                    if grid[i][jnum].is_ascii_digit() {
                        num.push(grid[i][jnum]);
                        visited[i][jnum] = true;
                    } else {
                        break;
                    }
                }
                let found_num: u32 = num
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("Failed to parse");
                numbers.push(Number {
                    number: found_num,
                    x: j as i32,
                    y: i as i32,
                    tagged: false,
                })
            }
        }
    }

    // Search for tags around each number
    numbers
        .iter_mut()
        .for_each(|num| get_surrounding_squares_tag(&grid, num));

    let output = numbers
        .iter()
        .filter(|num| num.tagged)
        .fold(0, |acc, n| acc + n.number);

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..\n\
                           ...*......\n\
                           ..35..633.\n\
                           ......#...\n\
                           617*......\n\
                           .....+.58.\n\
                           ..592.....\n\
                           ......755.\n\
                           ...$.*....\n\
                           .664.598..";
        assert_eq!("4361", process(input))
    }
}
