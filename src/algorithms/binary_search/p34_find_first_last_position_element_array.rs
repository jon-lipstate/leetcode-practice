// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
// If target is not found in the array, return [-1, -1].
// You must write an algorithm with O(log n) runtime complexity.

// Constraints:
// 0 <= nums.length <= 10^5
// -10^9 <= nums[i] <= 10^9
// nums is a non-decreasing array.
// -10^9 <= target <= 10^9

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![-1, -1];
    }
    //  else if nums.len() == 1 && nums[0] == target {
    //     return vec![0, 0];
    // }
    let mut l = 0;
    let mut h = nums.len() - 1;
    let mut last_iter = false;
    while h >= l && !last_iter {
        if h - l == 0 {
            last_iter = true;
        }
        let m = l + (h - l) / 2;
        match nums[m].cmp(&target) {
            std::cmp::Ordering::Equal => {
                let mut max = m;
                let mut min = m;

                for i in m..(h + 1) {
                    if nums[i] == target {
                        max = i;
                    } else {
                        break;
                    }
                }
                for i in (l..m).rev() {
                    if nums[i] == target {
                        min = i;
                    } else {
                        break;
                    }
                }

                return vec![min as i32, max as i32];
            }
            std::cmp::Ordering::Greater => {
                h = m;
            }
            std::cmp::Ordering::Less => {
                l = m + 1;
            }
        }
    }
    vec![-1, -1]
}
#[test]
fn t5() {
    let nums = vec![1, 4];
    let t = 4;
    let r = search_range(nums, t);
    assert_eq!(vec![1, 1], r);
}
#[test]
fn t0() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let t = 8;
    let r = search_range(nums, t);
    assert_eq!(vec![3, 4], r);
}
#[test]
fn t1() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let t = 6;
    let r = search_range(nums, t);
    assert_eq!(vec![-1, -1], r);
}
#[test]
fn t2() {
    let nums = vec![];
    let t = 0;
    let r = search_range(nums, t);
    assert_eq!(vec![-1, -1], r);
}

#[test]
fn t3() {
    let nums = vec![1, 1, 1, 1];
    let t = 1;
    let r = search_range(nums, t);
    assert_eq!(vec![0, 3], r);
}
#[test]
fn t4() {
    let nums = vec![1];
    let t = 1;
    let r = search_range(nums, t);
    assert_eq!(vec![0, 0], r);
}
#[test]
fn t6() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let t = 6;
    let r = search_range(nums, t);
    assert_eq!(vec![-1, -1], r);
}
