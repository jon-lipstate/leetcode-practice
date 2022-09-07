// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

// You must write an algorithm with O(log n) runtime complexity.

// Example 1:

// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
// Example 2:

// Constraints:
// 1 <= nums.length <= 10^4
// -10^4 < nums[i], target < 10^4
// All the integers in nums are unique.
// nums is sorted in ascending order.

use std::cmp::Ordering;
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;

    while left <= right {
        let mid = (left + right) / 2;

        match target.cmp(&nums[mid as usize]) {
            Ordering::Equal => return mid as i32,
            Ordering::Greater => left = mid + 1,
            Ordering::Less => right = mid - 1,
        }
    }

    -1
}

#[test]
fn t0() {
    let nums = [-1, 0, 3, 5, 9, 12].to_vec();
    let target = 9;
    let expect = 4;
    let result = search(nums, target);
    assert_eq!(expect, result);
}
#[test]
fn t1() {
    let nums = [-1, 0, 3, 5, 9, 12].to_vec();
    let target = 2;
    let expect = -1;
    let result = search(nums, target);
    assert_eq!(expect, result);
}
#[test]
fn t2() {
    let nums = [].to_vec();
    let target = 2;
    let expect = -1;
    let result = search(nums, target);
    assert_eq!(expect, result);
}
#[test]
fn t3() {
    let nums = [-1, 0, 3, 5, 9, 12].to_vec();
    let target = -1;
    let expect = 0;
    let result = search(nums, target);
    assert_eq!(expect, result);
}
#[test]
fn t4() {
    let nums = [-1, 0, 3, 5, 9, 12].to_vec();
    let target = 12;
    let expect = 5;
    let result = search(nums, target);
    assert_eq!(expect, result);
}
#[test]
fn t5() {
    let nums = [5].to_vec();
    let target = 5;
    let expect = 0;
    let result = search(nums, target);
    assert_eq!(expect, result);
}
