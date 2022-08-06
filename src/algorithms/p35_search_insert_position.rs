// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
// You must write an algorithm with O(log n) runtime complexity.

// Constraints:
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums contains distinct values sorted in ascending order.
// -104 <= target <= 104

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //clear empty, and GT/LT Arry Size:
    if nums.len() == 0 {
        return 0;
    } else if target > nums[nums.len() - 1] {
        return nums.len() as i32;
    } else if target < nums[0] {
        return 0;
    }
    let mut left = 0;
    let mut right = nums.len() - 1;
    loop {
        let mid = (left + right) / 2;
        let last_entry = right - left == 1;
        // println!("L: {left}, R: {right}, Mid: {mid}");
        match target.cmp(&nums[mid]) {
            std::cmp::Ordering::Equal => {
                // println!("t: {target} == {:?}", nums[mid]);
                return mid as i32;
            }
            std::cmp::Ordering::Less => {
                // println!("t: {target} < {:?}", nums[mid]);
                right = mid;
                if last_entry {
                    return mid as i32;
                }
            }
            std::cmp::Ordering::Greater => {
                // println!("t: {target} > {:?}", nums[mid]);
                left = mid;
                if last_entry {
                    return mid as i32 + 1;
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
