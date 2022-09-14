// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.

// Constraints:
// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 1000

// Follow up:
// What if the given array is already sorted? How would you optimize your algorithm?
// What if nums1's size is small compared to nums2's size? Which algorithm is better?
// What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

#[test]
fn t1() {
    let nums1 = [1, 2, 2, 1].to_vec();
    let nums2 = [2, 2].to_vec();
    assert_eq!(vec![2, 2], intersect(nums1, nums2));
}
#[test]
fn t2() {
    let nums1 = [4, 9, 5].to_vec();
    let nums2 = [9, 4, 9, 8, 4].to_vec();
    // Explanation: [9,4] is also accepted.
    assert_eq!(vec![4, 9], intersect(nums1, nums2));
}
