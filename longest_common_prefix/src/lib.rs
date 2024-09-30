trait CommonPrefix {
    fn common_prefix(&self, str: &Self) -> Self;
}

impl CommonPrefix for String {
    fn common_prefix(&self, str: &String) -> String {
        self.chars()
            .zip(str.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect()
    }
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.iter()
        .skip(1)
        .fold(strs[0].clone(), |acc, str| acc.common_prefix(str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ]);
        assert_eq!(result, String::from("fl"));
    }
    #[test]
    fn test2() {
        let result = longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ]);
        assert_eq!(result, String::from(""));
    }
}
