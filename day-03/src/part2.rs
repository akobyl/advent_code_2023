use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum GearOrNot {
    None,
    GearLoc(i32, i32),
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Gear {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Number {
    number: u32,
    x: i32,
    y: i32,
    gear: GearOrNot,
}

fn is_geared(grid: &[Vec<char>], number: &mut Number) {
    let size = number.number.to_string().len() as i32;

    for i in number.y - 1..=number.y + 1 {
        if i < 0 || i >= grid.len() as i32 {
            continue;
        }

        for j in number.x - 1..=number.x + size {
            if j < 0 || j >= grid[0].len() as i32 {
                continue;
            }
            if grid[i as usize][j as usize] == '*' {
                number.gear = GearOrNot::GearLoc(j, i);
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
    let mut gears: Vec<Gear> = Vec::new();

    // find gears
    for (i, row) in grid.iter().enumerate() {
        for (j, &pixel) in row.iter().enumerate() {
            if pixel == '*' {
                gears.push(Gear {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }

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
                    gear: GearOrNot::None,
                })
            }
        }
    }

    // Process the numbers to identify gears
    numbers.iter_mut().for_each(|n| is_geared(&grid, n));

    // Filter out non * numbers
    let filtered_numbers: Vec<&Number> = numbers
        .iter()
        .filter(|&n| matches!(n.gear, GearOrNot::GearLoc(_, _)))
        .collect();

    // Create a hashmap of gears with bordering numbers
    let mut gearmap: HashMap<Gear, Vec<u32>> = HashMap::new();
    filtered_numbers.iter().for_each(|n| {
        let (x, y) = match n.gear {
            GearOrNot::GearLoc(x, y) => (x, y),
            _ => (0, 0),
        };
        gearmap.entry(Gear { x, y }).or_default().push(n.number);
    });

    // Filter only gears with 2 numbers
    // Return just the vec of numbers
    // Fold to accumulate the product of those vecs
    let sum = gearmap
        .iter()
        .filter(|&(_, val)| val.len() == 2)
        .map(|(_, val)| val)
        .fold(0, |acc: u32, v| acc + v.iter().product::<u32>());

    sum.to_string()
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
        assert_eq!("467835", process(input))
    }
}
