#[derive(Debug, Clone, PartialEq)]
struct Lens {
    label: String,
    focal: u8,
}

impl Lens {
    fn from_str(input: &str) -> Self {
        let end_index = input.find('-').or(input.find('=')).unwrap();
        Self {
            label: input[..end_index].to_string(),
            focal: input[end_index + 1..].parse().unwrap(),
        }
    }
}

fn hash_string(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn box_focus_power(box_id: usize, lens_box: &[Lens]) -> u64 {
    lens_box.iter().enumerate().fold(0, |acc, (slot, lens)| {
        acc + ((box_id as u64 + 1) * (slot as u64 + 1) * lens.focal as u64)
    })
}

pub fn process(input: &str) -> String {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    let steps: Vec<_> = input.split(',').collect();

    for step in steps {
        let end_index = step.find('-').or(step.find('=')).unwrap();
        let label = &step[..end_index];
        let box_id = hash_string(label) as usize;
        let lens_box = boxes.get_mut(box_id).unwrap();

        if step.contains('-') {
            lens_box.retain(|lens| lens.label != label);
        } else if step.contains('=') {
            let lens = Lens::from_str(step);
            let lens_index = lens_box.iter().position(|l| l.label == lens.label);

            if let Some(index) = lens_index {
                lens_box[index] = lens;
            } else {
                lens_box.push(lens);
            }
        } else {
            panic!("unknown action: {}", step);
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .fold(0, |acc, (e, lens_box)| acc + box_focus_power(e, lens_box));
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input))
    }

    #[test]
    fn test_lens_from_str_1() {
        assert_eq!(
            Lens::from_str("rn=1"),
            Lens {
                label: "rn".to_string(),
                focal: 1
            }
        );
    }

    #[test]
    fn test_lens_from_str_2() {
        assert_eq!(
            Lens::from_str("qp=3"),
            Lens {
                label: "qp".to_string(),
                focal: 3
            }
        );
    }

    #[test]
    fn test_box_focus_power_1() {
        let box0 = vec![Lens::from_str("rn=1"), Lens::from_str("cm=2")];
        assert_eq!(box_focus_power(0, &box0), 5);
    }

    #[test]
    fn test_box_focus_power_2() {
        let box3 = vec![
            Lens::from_str("ot=7"),
            Lens::from_str("ab=5"),
            Lens::from_str("pc=6"),
        ];
        assert_eq!(box_focus_power(3, &box3), 28 + 40 + 72);
    }
}
