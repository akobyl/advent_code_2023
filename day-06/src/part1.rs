fn generate_races(input: &str) -> Vec<(u32, u32)> {
    let times: Vec<u32> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    assert_eq!(times.len(), distances.len());
    let output: Vec<_> = times
        .iter()
        .copied()
        .zip(distances.iter().copied())
        .collect();
    output
}
pub fn process(input: &str) -> String {
    let races = generate_races(input);
    let output: u32 = races
        .iter()
        .map(|race| (1..race.0).filter(|&i| i * (race.0 - i) > race.1).count() as u32)
        .product();

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_races() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        let races = generate_races(input);
        assert_eq!(races.first(), Some(&(7, 9)));
        assert_eq!(races.get(1), Some(&(15, 40)));
        assert_eq!(races.get(2), Some(&(30, 200)));
        assert_eq!(races.get(3), None);
    }
    #[test]
    fn test_process() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200";
        assert_eq!("288", process(input))
    }
}
