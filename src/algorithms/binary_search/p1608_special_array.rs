// You are given an array nums of non-negative integers. nums is considered special if there exists a number x such that there are exactly x numbers in nums that are greater than or equal to x.
// Notice that x does not have to be an element in nums.
// Return x if the array is special, otherwise, return -1. It can be proven that if nums is special, the value for x is unique.

// Constraints:
// 1 <= nums.length <= 100
// 0 <= nums[i] <= 1000
pub fn special_array(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut l = 0;
    let mut r = nums.len();
    while l <= r {
        let m = l + (r - l) / 2;
        let gte = count(&nums, m as i32);
        match (m as i32).cmp(&gte) {
            std::cmp::Ordering::Equal => {
                return m as i32;
            }
            std::cmp::Ordering::Greater => {
                r = m - 1;
            }
            std::cmp::Ordering::Less => {
                l = m + 1;
            }
        }
    }
    -1
}

fn count(nums: &Vec<i32>, t: i32) -> i32 {
    let mut gte = 0;
    for n in nums {
        if n >= &t {
            gte += 1;
        }
    }
    gte
}

#[test]
fn t0() {
    // Explanation: There are 2 values (3 and 5) that are greater than or equal to 2.
    assert_eq!(2, special_array(vec![3, 5]));
}
#[test]
fn t1() {
    // Explanation: No numbers fit the criteria for x.
    // If x = 0, there should be 0 numbers >= x, but there are 2.
    // If x = 1, there should be 1 number >= x, but there are 0.
    // If x = 2, there should be 2 numbers >= x, but there are 0.
    // x cannot be greater since there are only 2 numbers in nums.
    assert_eq!(-1, special_array(vec![0, 0]));
}
#[test]
fn t2() {
    // Explanation: There are 3 values that are greater than or equal to 3.

    assert_eq!(3, special_array(vec![0, 4, 3, 0, 4]));
}
