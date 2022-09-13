// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
// The tests are generated such that there is exactly one solution. You may not use the same element twice.
// Your solution must use only constant extra space.

// Constraints:
// 2 <= numbers.length <= 3 * 10^4
// -1000 <= numbers[i] <= 1000
// numbers is sorted ascending.
// -1000 <= target <= 1000
// The tests are generated such that there is exactly one solution.

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..numbers.len() {
        let ni = numbers[i];
        let need = target - ni;
        let l;
        let r;
        if need > ni {
            l = i;
            r = numbers.len();
        } else {
            l = 0;
            r = i;
        }
        if let Some(m) = bin_srch(&numbers, l, r, need) {
            let min = i32::min(i as i32, m);
            let max = i32::max(i as i32, m);
            return vec![min + 1, max + 1];
        }
    }
    unreachable!()
}
fn bin_srch(numbers: &Vec<i32>, l: usize, r: usize, target: i32) -> Option<i32> {
    let mut l = l;
    let mut r = r;
    while l < r {
        let m = l + (r - l) / 2;
        match numbers[m].cmp(&target) {
            std::cmp::Ordering::Equal => {
                return Some(m as i32);
            }
            std::cmp::Ordering::Greater => {
                r = m;
            }
            std::cmp::Ordering::Less => {
                l = m + 1;
            }
        }
    }
    None
}

#[test]
fn t0() {
    // Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
    assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
}
#[test]
fn t1() {
    // Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
    assert_eq!(vec![1, 3], two_sum(vec![2, 3, 4], 6));
}
#[test]
fn t2() {
    // Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
    assert_eq!(vec![1, 2], two_sum(vec![-1, 0], -1));
}
