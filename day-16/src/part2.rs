pub fn process(_input: &str) -> String {
    println!("hi part 2");
    "part 2".to_string()
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
