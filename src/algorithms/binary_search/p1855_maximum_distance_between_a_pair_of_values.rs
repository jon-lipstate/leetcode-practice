// You are given two non-increasing 0-indexed integer arrays nums1​​​​​​ and nums2​​​​​​.
// A pair of indices (i, j), where 0 <= i < nums1.length and 0 <= j < nums2.length, is valid if both i <= j and nums1[i] <= nums2[j]. The distance of the pair is j - i​​​​.
// Return the maximum distance of any valid pair (i, j). If there are no valid pairs, return 0.
// An array arr is non-increasing if arr[i-1] >= arr[i] for every 1 <= i < arr.length.

// Constraints:
// 1 <= nums1.length, nums2.length <= 10^5
// 1 <= nums1[i], nums2[j] <= 10^5
// Both nums1 and nums2 are non-increasing.

pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    unimplemented!()
}

#[test]
fn t1() {
    // Explanation: The valid pairs are (0,0), (2,2), (2,3), (2,4), (3,3), (3,4), and (4,4).
    // The maximum distance is 2 with pair (2,4).
    let nums1 = [55, 30, 5, 4, 2].to_vec();
    let nums2 = [100, 20, 10, 10, 5].to_vec();
    assert_eq!(2, max_distance(nums1, nums2));
}
#[test]
fn t2() {
    // Explanation: The valid pairs are (0,0), (0,1), and (1,1).
    // The maximum distance is 1 with pair (0,1).
    let nums1 = [2, 2, 2].to_vec();
    let nums2 = [10, 10, 1].to_vec();
    assert_eq!(1, max_distance(nums1, nums2));
}
#[test]
fn t3() {
    // Explanation: The valid pairs are (2,2), (2,3), (2,4), (3,3), and (3,4).
    // The maximum distance is 2 with pair (2,4).
    let nums1 = [30, 29, 19, 5].to_vec();
    let nums2 = [25, 25, 25, 25, 25].to_vec();
    assert_eq!(2, max_distance(nums1, nums2));
}
