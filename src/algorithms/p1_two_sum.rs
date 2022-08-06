// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

// Constraints:
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h = HashMap::new();
    for idx in 0..nums.len() {
        let diff = target - nums[idx];
        match h.entry(diff) {
            Occupied(e) => {
                let i2 = e.get();
                return vec![idx as i32, *i2 as i32];
            }
            Vacant(_) => {
                h.insert(nums[idx], idx);
            }
        }
    }
    unreachable!()
}

#[test]
fn test0() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    // Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    let ts = two_sum(nums, target);
    println!("{:?} s/b [0,1]", ts);
}

#[test]
fn test1() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let ts = two_sum(nums, target);
    println!("{:?} s/b [1,2]", ts);
}
#[test]
fn test2() {
    let nums = vec![3, 3];
    let target = 6;
    let ts = two_sum(nums, target);
    println!("{:?} s/b [0,1]", ts);
}
