// There is an integer array nums sorted in ascending order (with distinct values).
// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
// You must write an algorithm with O(log n) runtime complexity.

// Constraints:
// 1 <= nums.length <= 5000
// -10^4 <= nums[i] <= 10^4
// All values of nums are unique.
// nums is an ascending array that is possibly rotated.
// -10^4 <= target <= 10^4
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    unimplemented!()
}

#[test]
fn t1() {
    let nums = [4, 5, 6, 7, 0, 1, 2].to_vec();
    assert_eq!(4, search(nums, 0));
}
#[test]
fn t2() {
    let nums = [4, 5, 6, 7, 0, 1, 2].to_vec();
    assert_eq!(-1, search(nums, 3));
}
#[test]
fn t3() {
    let nums = [1].to_vec();
    assert_eq!(-1, search(nums, 0));
}
