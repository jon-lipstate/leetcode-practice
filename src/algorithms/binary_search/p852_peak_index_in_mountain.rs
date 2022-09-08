// An array arr a mountain if the following properties hold:
// arr.length >= 3
// There exists some i with 0 < i < arr.length - 1 such that:
// arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
// arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
// Given a mountain array arr, return the index i such that arr[0] < arr[1] < ... < arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1].

// You must solve it in O(log(arr.length)) time complexity.

// Constraints:
// 3 <= arr.length <= 10^5
// 0 <= arr[i] <= 10^6
// arr is guaranteed to be a mountain array.

pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut low_idx = 0;
    let mut high_idx = arr.len() - 1;
    loop {
        let mid_idx = (low_idx + high_idx) / 2;
        let mid_val = arr[mid_idx];
        let mid_p1 = arr[mid_idx + 1];
        let mid_m1 = arr[mid_idx - 1];
        if mid_val > mid_m1 && mid_val > mid_p1 {
            return mid_idx as i32;
        } else if mid_val > mid_m1 {
            low_idx = mid_idx;
        } else if mid_val > mid_p1 {
            high_idx = mid_idx;
        }
    }
}

#[test]
fn tt() {
    let arr = [0, 1, 2, 5, 7, 9, 11, 0].to_vec();
    let r = peak_index_in_mountain_array(arr);
    assert_eq!(6, r);
}

#[test]
fn t0() {
    let arr = [0, 1, 0].to_vec();
    let r = peak_index_in_mountain_array(arr);
    assert_eq!(1, r);
}
#[test]
fn t1() {
    let arr = [0, 2, 1, 0].to_vec();
    let r = peak_index_in_mountain_array(arr);
    assert_eq!(1, r);
}

#[test]
fn t2() {
    let arr = [0, 10, 5, 2].to_vec();
    let r = peak_index_in_mountain_array(arr);
    assert_eq!(1, r);
}
#[test]
fn t3() {
    let arr = [3, 5, 3, 2, 0].to_vec();
    let r = peak_index_in_mountain_array(arr);
    assert_eq!(1, r);
}
