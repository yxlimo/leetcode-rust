/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */
use std::collections::HashMap;

pub struct Solution;

impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut h: HashMap<i32, i32> = HashMap::new();
		for (i, num) in nums.iter().enumerate() {
			let left = target - num;
			match h.get(&left) {
				Some(v) => return vec![*v, i as i32],
				None => h.insert(*num, i as i32),
			};
		}
		vec![]
	}
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }
}