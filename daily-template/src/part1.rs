pub fn process(_input: &str) -> String {
    println!("hi part 1");
    "part 1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "";
        assert_eq!("", process(input))
    }
}
