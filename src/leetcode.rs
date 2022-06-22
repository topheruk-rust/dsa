/// https://neetcode.io/
pub struct Solution;

impl Solution {
    /// https://leetcode.com/problems/two-sum/
    /// Given an array of integers nums and an integer target,
    /// return indices of the two numbers such that they add up to target.
    pub fn problem_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap as hm;
        nums.iter()
            .enumerate()
            .fold((vec![], hm::new()), |(vec, mut map), (pos, val)| {
                map.insert(val, pos as i32);
                match map.get(&(target - val)) {
                    Some(&pos_cached) => (vec![pos_cached, pos as i32], map),
                    None => (vec, map),
                }
            })
            .0
    }

    /// https://leetcode.com/problems/contains-duplicate/
    /// Given an integer array nums,
    /// return true if any value appears at least twice in the array,
    /// and return false if every element is distinct.
    pub fn problem_217(nums: Vec<i32>) -> bool {
        let mut hs = std::collections::HashSet::new();
        !nums.iter().all(|x| hs.insert(x))
    }

    /// https://leetcode.com/problems/valid-anagram/
    /// Given two strings s and t,
    /// return true if t is an anagram of s,
    /// and false otherwise.
    pub fn problem_242(s: String, t: String) -> bool {
        use std::collections::HashMap as hm;

        if s.len() == t.len() {
            s.chars()
                .zip(t.chars())
                .fold(hm::new(), |mut map, (x, y)| {
                    *map.entry(x).or_insert(0) += 1;
                    *map.entry(y).or_insert(0) -= 1;
                    map
                })
                .values()
                .any(|&x| x != 0)
        } else {
            false
        }
    }
}
