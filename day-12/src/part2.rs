#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    fn from_char(c: char) -> Status {
        match c {
            '?' => Status::Unknown,
            '#' => Status::Damaged,
            _ => Status::Operational,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Group {
    springs: Vec<Status>,
    brokens: Vec<u32>,
}

impl Group {
    fn from_string(s: &str) -> Group {
        let brokens: Vec<_> = s
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let group: Vec<_> = s
            .split_whitespace()
            .next()
            .unwrap()
            .chars()
            .map(Status::from_char)
            .collect();

        Group {
            springs: group,
            brokens,
        }
    }

    fn expand(self) -> Group {
        let mut springs_expanded = self.springs;
        springs_expanded.push(Status::Unknown);
        let total_len = springs_expanded.len() * 5 - 1;
        let springs_expanded = springs_expanded
            .iter()
            .cycle()
            .take(total_len)
            .cloned()
            .collect();

        let total_len = self.brokens.len() * 5;
        let brokens_expanded = self
            .brokens
            .iter()
            .cycle()
            .take(total_len)
            .cloned()
            .collect();

        Group {
            springs: springs_expanded,
            brokens: brokens_expanded,
        }
    }
}

// Inspired by https://qsantos.fr/2024/01/04/dynamic-programming-is-not-black-magic/
fn count_arrangements(group: Group) -> usize {
    let mut cache = Vec::new();

    {
        let mut first_row = Vec::new();
        first_row.push(1);
        first_row.resize(group.brokens.len() + 1, 0);
        cache.push(first_row);
    }

    for i in 1..=group.springs.len() {
        let spring = group.springs[i - 1];
        let mut row = Vec::new();

        for j in 0..=group.brokens.len() {
            let mut count = 0;

            // No group gets used
            if spring == Status::Operational || spring == Status::Unknown {
                count += cache[i - 1][j];
            }

            // Use a single group
            if j > 0 {
                let group_size = group.brokens[j - 1] as usize;
                if i >= group_size
                    && group.springs[(i - 1) - (group_size - 1)..=(i - 1)]
                        .iter()
                        .all(|s| *s == Status::Damaged || *s == Status::Unknown)
                {
                    if i == group_size {
                        count += cache[0][j - 1];
                    } else {
                        let s = group.springs[(i - 1) - (group_size - 1) - 1];
                        if s == Status::Operational || s == Status::Unknown {
                            count += cache[i - group_size - 1][j - 1];
                        }
                    }
                }
            }
            row.push(count);
        }
        cache.push(row);
    }
    cache[group.springs.len()][group.brokens.len()]
}

pub fn process(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let g = Group::from_string(line).expand();

        sum += count_arrangements(g);
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_1() {
        let input = ".# 1";
        let g = Group::from_string(input);

        let expected = ".#?.#?.#?.#?.# 1,1,1,1,1";
        let g_expected = Group::from_string(expected);
        let expanded = g.expand();
        assert_eq!(expanded, g_expected);
    }

    #[test]
    fn test_expand_2() {
        let input = "???.### 1,1,3";
        let g = Group::from_string(input).expand();

        let expected = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";
        let g_expected = Group::from_string(expected);
        assert_eq!(g, g_expected);
    }

    #[test]
    fn test() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("525152", process(input));
    }
}
