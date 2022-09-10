// Given a non-negative integer x, compute and return the square root of x.
// Since the return type is an integer, the decimal digits are truncated, and only the integer part of the result is returned.
// Note: You are not allowed to use any built-in exponent function or operator, such as pow(x, 0.5) or x ** 0.5.

// Constraints:
// 0 <= x <= 2^31 - 1

pub fn my_sqrt(x: i32) -> i32 {
    let mut l = 0;
    let mut r = i32::min(x, 46340);
    let x = x as i64;
    loop {
        let m = (l + (r - l) / 2) as i64;
        let mm = m * m;
        let mmp1 = (m + 1) * (m + 1);

        let m_lt_x = mm < x;
        let mp_gt_x = mmp1 > x;

        if mm == x || m_lt_x && mp_gt_x {
            return m as i32;
        } else if m_lt_x && !mp_gt_x {
            l = (m + 1) as i32;
        } else if !m_lt_x && mp_gt_x {
            r = m as i32;
        }
    }
}

#[test]
fn ts0() {
    //Explanation: The square root of 8 is 2.82842..., and since the decimal part is truncated, 2 is returned.
    assert_eq!(0, my_sqrt(0));
}
#[test]
fn t0() {
    assert_eq!(1, my_sqrt(1));
}
#[test]
fn t1() {
    assert_eq!(2, my_sqrt(4));
}
#[test]
fn t2() {
    //Explanation: The square root of 8 is 2.82842..., and since the decimal part is truncated, 2 is returned.
    assert_eq!(2, my_sqrt(8));
}
#[test]
fn t3() {
    assert_eq!(46340, my_sqrt(2_147_483_647));
}
