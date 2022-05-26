use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
    }

    pub fn problem_740(nums: Vec<i32>) -> i32 {
        let (a, b) = nums
            .iter()
            .fold(BTreeMap::new(), |mut m, n| {
                *m.entry(n).or_insert(0) += 1;
                m
            })
            .iter()
            .fold((0, 0), |(a, b), (k, v)| match true {
                _ if a == *k - 1 => (a, b),
                _ => (b, b + v),
            });

        a.max(b)
    }

    /// https://leetcode.com/problems/n-th-tribonacci-number/
    pub fn problem_1137(n: i32) -> i32 {
        (0..n).fold((0, 1, 1), |(a, b, c), _| (b, c, a + b + c)).0
    }

    /// https://leetcode.com/problems/house-robber/
    pub fn problem_198(nums: Vec<i32>) -> i32 {
        let (a, b) = nums.iter().fold((0, 0), |(a, b), n| (a.max(b), a + n));
        a.max(b)
    }

    /// https://leetcode.com/problems/min-cost-climbing-stairs/
    pub fn problem_746(cost: Vec<i32>) -> i32 {
        cost.iter()
            .chain(std::iter::once(&0))
            .fold((0, 0), |(x, y), n| (y, n + x.min(y)))
            .1
    }

    /// https://leetcode.com/problems/fibonacci-number/
    pub fn problem_509(n: i32) -> i32 {
        (0..n).fold((0, 1), |(a, b), _| (a, a + b)).0
    }

    /// https://leetcode.com/problems/contains-duplicate/
    pub fn problem_217<T>(nums: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut m = HashSet::new();
        !nums.into_iter().all(|x| m.insert(x))
    }

    /// https://leetcode.com/problems/climbing-stairs/
    pub fn problem_70(n: i32) -> i32 {
        let g = (1.0 + 5.0_f64.sqrt()) / 2.0;
        ((g.powi(n + 1) - (1.0 - g).powi(n + 1)) / 5.0_f64.sqrt()).round() as i32
    }

    /// https://leetcode.com/problems/maximum-subarray/
    pub fn problem_53(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((i32::MIN, 0), |(best_sum, curr_sum), n| {
                let v = curr_sum + n;
                (best_sum.max(v), 0.max(v))
            })
            .0
    }

    /// https://leetcode.com/problems/merge-sorted-array/
    pub fn problem_88(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {}

    /// https://leetcode.com/problems/two-sum/
    pub fn problem_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .fold((HashMap::new(), vec![]), |(mut m, mut v), (i, n)| {
                if let Some(&j) = m.get(&(target - n)) {
                    v.push(j);
                    v.push(i as i32);
                };

                m.insert(n, i as i32);

                (m, v)
            })
            .1
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::fib(7), 13);
    }

    #[test]
    fn test_746() {
        assert_eq!(Solution::problem_746(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::problem_746(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::problem_1(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::problem_1(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::problem_1(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_53() {
        assert_eq!(Solution::problem_53(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(Solution::problem_53(vec![1]), 1);
        assert_eq!(Solution::problem_53(vec![-1]), -1);
        assert_eq!(Solution::problem_53(vec![5, 4, -1, 7, 8]), 23)
    }

    #[test]
    fn test_88() {
        let nums1 = &mut vec![1, 2, 3, 0, 0, 0];
        let nums2 = &mut vec![2, 5, 6];

        Solution::problem_88(nums1, 3, nums2, 3);

        assert_eq!(nums1, &mut vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_217() {
        assert!(Solution::problem_217(vec![1, 2, 3, 1]));
        assert!(!Solution::problem_217(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_70() {
        assert_eq!(Solution::problem_70(1), 1);
        assert_eq!(Solution::problem_70(6), 13);
        assert_eq!(Solution::problem_70(9), 55);
        assert_eq!(Solution::problem_70(13), 377);
        assert_eq!(Solution::problem_70(18), 4181);
    }

    #[test]
    fn test_509() {
        assert_eq!(Solution::problem_509(0), 0);
        assert_eq!(Solution::problem_509(2), 1);
        assert_eq!(Solution::problem_509(4), 3);
    }

    #[test]
    fn test_1137() {
        assert_eq!(Solution::problem_1137(4), 4);
        assert_eq!(Solution::problem_1137(25), 1389537);
    }
}
