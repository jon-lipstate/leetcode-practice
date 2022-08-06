// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.
// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
// Return k after placing the final result in the first k slots of nums.
// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

// ```
// Custom Judge:
// The judge will test your solution with the following code:
// int[] nums = [...]; // Input array
// int[] expectedNums = [...]; // The expected answer with correct length
// int k = removeDuplicates(nums); // Calls your implementation
// assert k == expectedNums.length;
// for (int i = 0; i < k; i++) {
//     assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.
// ```

// Example 1:

// Input: nums = [1,1,2]
// Output: 2, nums = [1,2,_]
// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
// Example 2:

// Input: nums = [0,0,1,1,1,2,2,3,3,4]
// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).

// Constraints:

// 1 <= nums.length <= 3 * 104
// -100 <= nums[i] <= 100
// nums is sorted in non-decreasing order.
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }
    let mut unique = 0;
    for i in 1..nums.len() {
        println!("i: {i}, u: {unique}, ni:{}, nu:{}", nums[i], nums[unique]);
        println!("nums:{:?}", nums);
        if nums[unique] != nums[i] {
            unique += 1;
            let tmp = nums[unique];
            nums[unique] = nums[i];
        }
    }

    return unique as i32 + 1;
}

pub fn xremove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut unique = 0;
    for idx in 1..nums.len() {
        // println!("BEFORE {},{},  {:?}", unique, idx, nums);
        if nums[idx] != nums[unique] {
            unique += 1;
            nums[unique] = nums[idx];
        }
        // println!("AFTER {},{},  {:?}", unique, idx, nums);
    }

    return (unique + 1) as i32;
}
#[test]
fn test() {
    // let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    // let n = remove_duplicates(&mut v);

    let mut v = vec![1, 1, 2, 2, 5, 5];
    let n = remove_duplicates(&mut v);

    println!("N:{}", n);
    println!("v:{:?}", v);
}
// 0  1  2  3  4  5  6  7  8  9
//[1, 1, 2, 3]
//
