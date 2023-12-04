#[derive(Debug, PartialEq)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

fn parse_game_id(line: &str) -> u32 {
    line.split_once(':')
        .unwrap()
        .0
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_color_group(group: &str) -> RGB {
    group
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.split(' '))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .fold(
            RGB {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, (v, k)| {
                match k {
                    "red" => acc.red = v.parse().unwrap(),
                    "green" => acc.green = v.parse().unwrap(),
                    "blue" => acc.blue = v.parse().unwrap(),
                    _ => panic!("Invalid color: {}", k),
                }
                acc
            },
        )
}

#[tracing::instrument]
pub fn process(_input: &str) -> String {
    let mut output = 0;
    for line in _input.split('\n') {
        let mut valid = true;
        let game_id = parse_game_id(line);
        let colors = line.split_once(':').unwrap().1.trim();
        let rgbs: Vec<RGB> = colors
            .split(';')
            .map(|s| s.trim())
            .map(parse_color_group)
            .collect();

        for rgb in rgbs {
            if rgb.red > 12 || rgb.green > 13 || rgb.blue > 14 {
                valid = false;
                break;
            }
        }

        if valid {
            output += game_id;
        }
    }

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                           Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                           Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                           Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                           Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input))
    }

    #[test]
    fn test_parse_game_id() {
        assert_eq!(parse_game_id("Game 1: 3 blue, 4 red"), 1);
        assert_eq!(parse_game_id("Game 2: 1 blue, 2 green"), 2);
        assert_eq!(parse_game_id("Game 3: 8 green, 6 blue, 20 red"), 3);
        assert_eq!(parse_game_id("Game 4: 1 green, 3 red, 6 blue"), 4);
        assert_eq!(parse_game_id("Game 5: 6 red, 1 blue, 3 green"), 5);
    }

    #[test]
    fn test_parse_color_group() {
        assert_eq!(
            parse_color_group("1 blue, 2 green"),
            RGB {
                red: 0,
                green: 2,
                blue: 1
            }
        );

        assert_eq!(
            parse_color_group("3 green, 4 blue, 1 red"),
            RGB {
                red: 1,
                green: 3,
                blue: 4
            }
        );

        assert_eq!(
            parse_color_group(" 3 green, 15 blue, 14 red"),
            RGB {
                red: 14,
                green: 3,
                blue: 15
            }
        );
    }
}
