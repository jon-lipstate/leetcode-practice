// Given an array arr of integers, check if there exist two indices i and j such that :
// i != j
// 0 <= i, j < arr.length
// arr[i] == 2 * arr[j]

// Constraints:
// 2 <= arr.length <= 500
// -10^3 <= arr[i] <= 10^3
pub fn check_if_exist(arr: Vec<i32>) -> bool {
    unimplemented!()
}

#[test]
fn t1() {
    let arr = [10, 2, 5, 3].to_vec();
    // Explanation: For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
    assert_eq!(true, check_if_exist(arr));
}
#[test]
fn t2() {
    let arr = [3, 1, 7, 11].to_vec();
    // Explanation: There is no i and j that satisfy the conditions.
    assert_eq!(false, check_if_exist(arr));
}
