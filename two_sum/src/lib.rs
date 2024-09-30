pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();
    for i in 0..length - 1 {
        for j in i + 1..length {
            let a = nums[i];
            let b = nums[j];
            if a + b == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*; // == use crate::two_sum;

    #[test]
    fn test_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_3() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
