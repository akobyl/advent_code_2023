fn process_line(sequence: &[i64]) -> i64 {
    let mut diffs: Vec<Vec<i64>> = vec![sequence.to_vec()];
    loop {
        diffs.push(
            diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i64>>(),
        );

        if diffs.last().unwrap().iter().all(|x| *x == 0) {
            break;
        }
    }

    diffs
        .iter()
        .rev()
        .skip(1)
        .fold(0, |acc, row| acc + row.last().unwrap())
}

pub fn process(input: &str) -> String {
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect();

    let sum = lines
        .iter()
        .fold(0i64, |acc, line| acc + process_line(line));

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15\n\
                     1 3 6 10 15 21\n\
                     10 13 16 21 30 45";
        assert_eq!("114", process(input))
    }
}
