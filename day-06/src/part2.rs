fn generate_race(input: &str) -> (u64, u64) {
    let time_str: String = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .skip(6)
        .filter(|c| !c.is_whitespace())
        .collect();

    let distance_str: String = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .skip(10)
        .filter(|c| !c.is_whitespace())
        .collect();

    (time_str.parse().unwrap(), distance_str.parse().unwrap())
}
pub fn process(input: &str) -> String {
    let race = generate_race(input);

    let output = (1..race.0).filter(|&i| i * (race.0 - i) > race.1).count() as u64;

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_races() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        let race = generate_race(input);
        assert_eq!(race, (71530, 940200));
    }
    #[test]
    fn test_process() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        assert_eq!("71503", process(input))
    }
}
