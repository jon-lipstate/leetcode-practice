// Given an array arr of integers, check if there exist two indices i and j such that :
// i != j
// 0 <= i, j < arr.length
// arr[i] == 2 * arr[j]

// Constraints:
// 2 <= arr.length <= 500
// -10^3 <= arr[i] <= 10^3
pub fn check_if_exist(arr: Vec<i32>) -> bool {
    let mut arr = arr;
    arr.sort();

    for i in 0..arr.len() {
        let f = arr[i];
        let (j, found) = find(&arr, f * 2);
        if found && i != j {
            return true;
        }
    }
    false
}

fn find(arr: &[i32], f: i32) -> (usize, bool) {
    let mut l = 0;
    let mut h = arr.len();
    while l < h {
        let m = l + (h - l) / 2;
        let v = arr[m];
        if v == f {
            return (m, true);
        } else if v > f {
            h = m;
        } else {
            l = m + 1;
        }
    }
    (999, false)
}

#[test]
fn t1() {
    let mut arr = [10, 2, 5, 3].to_vec();
    // Explanation: For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
    assert_eq!(true, check_if_exist(arr));
}
#[test]
fn t2() {
    let mut arr = [3, 1, 7, 11].to_vec();
    // Explanation: There is no i and j that satisfy the conditions.
    assert_eq!(false, check_if_exist(arr));
}
#[test]
fn t3() {
    let mut arr = [-2, 0, 10, -19, 4, 6, -8].to_vec();
    // Explanation: There is no i and j that satisfy the conditions.
    assert_eq!(false, check_if_exist(arr));
}
#[test]
fn t4() {
    let arr = [0, 0].to_vec();
    assert_eq!(true, check_if_exist(arr));
}
