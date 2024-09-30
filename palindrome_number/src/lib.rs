#[allow(dead_code)]
trait Reverse {
    fn reverse(&self) -> Self;
}

#[allow(dead_code)]
trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl<T: Reverse + Eq> Palindrome for T {
    fn is_palindrome(&self) -> bool {
        self.reverse() == *self
    }
}

impl Reverse for i32 {
    fn reverse(&self) -> Self {
        let mut reversed = 0;
        let mut tmp = match *self {
            x if x < 0 => -x,
            x => x,
        };
        while tmp != 0 {
            reversed = reversed * 10 + tmp % 10;
            tmp /= 10;
        }
        reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        let result = 121.is_palindrome();
        assert_eq!(result, true);
    }

    #[test]
    fn test_n121() {
        let result = (-121).is_palindrome();
        assert_eq!(result, false);
    }

    #[test]
    fn test_10() {
        let result = 10.is_palindrome();
        assert_eq!(result, false);
    }
}
