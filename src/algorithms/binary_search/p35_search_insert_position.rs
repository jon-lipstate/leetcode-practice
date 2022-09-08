// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// You must write an algorithm with O(log n) runtime complexity.

// Constraints:
// 1 <= nums.length <= 10^4
// -10^4 <= nums[i] <= 10^4
// nums contains distinct values sorted in ascending order.
// -10^4 <= target <= 10^4

use std::cmp::Ordering;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    } else if target > nums[nums.len() - 1] {
        return nums.len() as i32;
    } else if target < nums[0] {
        return 0;
    }
    let mut low: i32 = 0;
    let mut high: i32 = (nums.len() - 1) as i32;
    loop {
        let mid = (low + high) / 2;
        match &target.cmp(&nums[mid as usize]) {
            Ordering::Equal => {
                return mid as i32;
            }
            Ordering::Greater => {
                low = mid + 1;
                if high <= low {
                    return high as i32;
                }
            }
            Ordering::Less => {
                high = mid;
                if high <= low {
                    return low as i32;
                }
            }
        }
    }
}

#[test]
fn test1() {
    let nums = vec![1, 3, 5, 6];
    let t = 5;
    let r = search_insert(nums, t);
    assert_eq!(r, 2);
}
#[test]
fn test2() {
    let nums = vec![1, 3, 5, 6];
    let t = 2;
    println!("find {t} in {:?}", nums);
    let r = search_insert(nums, t);
    assert_eq!(r, 1);
}
#[test]
fn test3() {
    let nums = vec![1, 3, 5, 6];
    let t = 7;
    let r = search_insert(nums, t);
    assert_eq!(r, 4);
}
#[test]
fn test4() {
    let nums = vec![];
    let t = 0;
    let r = search_insert(nums, t);
    assert_eq!(r, 0);
}
#[test]
fn test5() {
    let nums = vec![1];
    let t = 3;
    let r = search_insert(nums, t);
    assert_eq!(r, 1);
}

#[test]
fn test6() {
    let nums = vec![1];
    let t = 0;
    let r = search_insert(nums, t);
    assert_eq!(r, 0);
}

#[test]
fn test7() {
    let nums = vec![1, 3];
    let t = 2;
    println!("find {t} in {:?}", nums);
    let r = search_insert(nums, t);
    assert_eq!(r, 1);
}

#[test]
fn test8() {
    let nums = vec![1, 3, 5];
    let t = 1;
    println!("find {t} in {:?}", nums);
    let r = search_insert(nums, t);
    assert_eq!(r, 0);
}
