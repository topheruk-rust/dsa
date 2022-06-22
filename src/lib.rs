mod leetcode;

#[cfg(test)]
mod tests {
    use super::leetcode::*;

    #[test]
    fn problem_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::problem_1(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn problem_242() {
        let (s, t) = ("apple", "ppla");
        let result = Solution::problem_242(s.to_string(), t.to_string());
        assert!(result)
    }
}
