// You have n coins and you want to build a staircase with these coins. The staircase consists of k rows where the ith row has exactly i coins. The last row of the staircase may be incomplete.
// Given the integer n, return the number of complete rows of the staircase you will build.

// Constraints:
// 1 <= n <= 23^1 - 1
pub fn arrange_coins(n: i32) -> i32 {
    if n < 3 {
        return 1;
    }
    if n == 3 {
        return 2;
    }
    let mut l = 2;
    let mut r = i32::min(n / 2, 92680);
    while l <= r {
        let m = l + (r - l) / 2;
        // area of triangle, +1 is to capture the next partial row
        let coins: i64 = m as i64 * (m as i64 + 1) / 2;
        if coins == n as i64 {
            return m;
        } else if coins < n as i64 {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    r
}

#[test]
fn t0() {
    let n = 5;
    // Explanation: Because the 3rd row is incomplete, we return 2.
    let expect = 2;
    assert_eq!(expect, arrange_coins(n));
}
#[test]
fn t1() {
    let n = 8;
    // Explanation: Because the 4th row is incomplete, we return 3.
    let expect = 3;
    assert_eq!(expect, arrange_coins(n));
}
#[test]
fn t2() {
    let n = 1;
    // Explanation: Because the 4th row is incomplete, we return 3.
    let expect = 1;
    assert_eq!(expect, arrange_coins(n));
}
#[test]
fn t3() {
    let n = 1804289383;
    let expect = 60070;
    assert_eq!(expect, arrange_coins(n));
}
