use std::ops::Index;

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
        let mut count = std::collections::HashSet::new();
        !nums.iter().all(|x| count.insert(x))
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
                .fold(hm::<char, i32>::new(), |mut map, (x, y)| {
                    *map.entry(x).or_default() += 1;
                    *map.entry(y).or_default() -= 1;
                    map
                })
                .values()
                .all(|&x| x == 0)
        } else {
            false
        }
    }

    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
    /// You want to maximize your profit by choosing a single
    /// day to buy one stock and choosing a different day
    /// in the future to sell that stock.
    pub fn problem_121(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(buy, profit), sell| {
                (buy.min(*sell), profit.max(sell - buy))
            })
            .1
    }

    /// https://leetcode.com/problems/product-of-array-except-self/
    /// Given an integer array nums,
    /// return an array answer such that answer[i] is equal
    /// to the product of all the elements of nums except nums[i].
    pub fn problem_238(nums: Vec<i32>) -> Vec<i32> {
        // Why is this O(n) & not O(n*n)?
        //
        // One loop: O(n)  Another loop: O(n)  Total -> O(2n). We remove the constant 2, so we have O(n).
        // In this case, we only ever traverse a given array twice. An array of a thousand elements is only two traversals.
        // If loops were nested, then we would make one extra traversal for every element in the array i.e. O(n*n).
        let _slow = nums
            .iter()
            .enumerate()
            .map(|(k, _)| {
                nums[..k].iter().product::<i32>() * nums[(k + 1)..].iter().product::<i32>()
            })
            .collect::<Vec<_>>();

        use std::mem::replace;

        let fast = nums
            .iter()
            .zip(
                nums.iter()
                    .rev()
                    .scan(1, |acc, n| Some(replace(acc, *acc * n)))
                    .collect::<Vec<_>>()
                    .iter()
                    .rev(),
            )
            .scan((1, 1), |(ml, _), (nl, nr)| {
                Some((replace(ml, *ml * nl), *nr))
            })
            .collect::<Vec<(_, _)>>()
            .iter()
            .map(|(a, b)| a * b)
            .collect::<Vec<_>>();

        fast
    }
}
