pub fn roman_to_int(s: String) -> i32 {
    s.chars().rfold(0, |acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iii() {
        let result = roman_to_int(String::from("III"));
        assert_eq!(result, 3);
    }
    #[test]
    fn lviii() {
        let result = roman_to_int(String::from("LVIII"));
        assert_eq!(result, 58);
    }
    #[test]
    fn mcmxciv() {
        let result = roman_to_int(String::from("MCMXCIV"));
        assert_eq!(result, 1994);
    }
}
