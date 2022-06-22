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
        let (s, t) = ("cat", "tac");
        let result = Solution::problem_242(s.to_string(), t.to_string());
        assert!(result)
    }

    #[test]
    fn problem_121() {
        let input = vec![7, 6, 4, 3, 1];
        let result = Solution::problem_121(input);
        assert_eq!(result, 0)
    }
}
