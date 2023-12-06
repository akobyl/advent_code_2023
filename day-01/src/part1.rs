#[tracing::instrument]
pub fn process(_input: &str) -> String {
    let mut sum = 0;
    for line in _input.split('\n') {
        let first_digit = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();

        // dbg!(first_digit, last_digit);
        sum += first_digit * 10 + last_digit
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2\n\
                    pqr3stu8vwx\n\
                    a1b2c3d4e5f\n\
                    treb7uchet";
        assert_eq!("142", process(input))
    }
}
