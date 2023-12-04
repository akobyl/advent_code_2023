#[tracing::instrument]
fn find_number(substring: &str) -> Option<u32> {
    let number_list = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_digit = substring
        .chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10));

    let first_word = number_list
        .iter()
        .enumerate()
        .find(|&(_, &num)| substring.contains(num))
        .map(|(index, _)| index as u32);

    first_digit.or(first_word)
}

#[tracing::instrument]
fn process_line(line: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;
    for len in 1..=line.len() {
        let substring = &line[..len];
        let num = find_number(substring);
        if num.is_some() {
            first = num.unwrap();
            break;
        }
    }

    // Get last
    for i in 1..=line.len() {
        let substring = &line[line.len() - i..];
        let num = find_number(substring);
        if num.is_some() {
            last = num.unwrap();
            break;
        }
    }

    first * 10 + last
}

pub fn process(_input: &str) -> String {
    let mut sum = 0;
    for line in _input.split("\n") {
        sum += process_line(line);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit_or_word() {
        let input = "two1nine";
        let answer = process_line(input);
        assert_eq!(29, answer);

        let input = "eightwothree";
        let answer = process_line(input);
        assert_eq!(83, answer);

        let input = "abcone2threexyz";
        let answer = process_line(input);
        assert_eq!(13, answer);

        let input = "xtwone3four";
        let answer = process_line(input);
        assert_eq!(24, answer);

        let input = "4nineeightseven2";
        let answer = process_line(input);
        assert_eq!(42, answer);

        let input = "zoneight234";
        let answer = process_line(input);
        assert_eq!(14, answer);

        let input = "7pqrstsixtee";
        let answer = process_line(input);
        assert_eq!(76, answer);

        let input = "four9four";
        let answer = process_line(input);
        assert_eq!(44, answer);
    }

    #[test]
    fn test_process() {
        let input = "two1nine\n\
            eightwothree3\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen";
        assert_eq!("281", process(input))
    }
}
