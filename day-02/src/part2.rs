#[derive(Debug, PartialEq)]
struct RGB {
    red: u32,
    green: u32,
    blue: u32,
}

impl RGB {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
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
        let colors = line.split_once(':').unwrap().1.trim();
        let rgbs: Vec<RGB> = colors
            .split(';')
            .map(|s| s.trim())
            .map(parse_color_group)
            .collect();

        let peak = rgbs.iter().fold(
            RGB {
                red: 0,
                green: 0,
                blue: 0,
            },
            |max_rgb, rgb| RGB {
                red: max_rgb.red.max(rgb.red),
                green: max_rgb.green.max(rgb.green),
                blue: max_rgb.blue.max(rgb.blue),
            },
        );
        output += peak.power();
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
        assert_eq!("2286", process(input))
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

    #[test]
    fn test_power() {
        assert_eq!(
            RGB {
                red: 0,
                green: 2,
                blue: 1
            }
            .power(),
            0
        );
        assert_eq!(
            RGB {
                red: 4,
                green: 2,
                blue: 6
            }
            .power(),
            48
        );
    }
}
