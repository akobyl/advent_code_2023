fn hash_string(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

pub fn process(input: &str) -> String {
    let sum: u32 = input.split(',').map(hash_string).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input))
    }

    #[test]
    fn test_hash_string_1() {
        assert_eq!(52, hash_string("HASH"));
    }

    #[test]
    fn hash_string_2() {
        assert_eq!(30, hash_string("rn=1"));
    }
}
