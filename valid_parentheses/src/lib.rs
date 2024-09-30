use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut parenthesis = Vec::new();
    let pairs = HashMap::from([
      (')', '('),
      ('}', '{'),
      (']', '[')
    ]);

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => parenthesis.push(c),
            ')' | '}' | ']' => {
                if parenthesis.last() == pairs.get(&c) {
                    parenthesis.pop();
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    parenthesis.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = is_valid(String::from("()"));
        assert_eq!(result, true);
    }
    #[test]
    fn test2() {
        let result = is_valid(String::from("()[]{}"));
        assert_eq!(result, true);
    }
    #[test]
    fn test3() {
        let result = is_valid(String::from("(]"));
        assert_eq!(result, false);
    }
    #[test]
    fn test4() {
        let result = is_valid(String::from("([])"));
        assert_eq!(result, true);
    }
}
