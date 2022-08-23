// Given a non-negative integer x, compute and return the square root of x.
// Since the return type is an integer, the decimal digits are truncated, and only the integer part of the result is returned.
// Note: You are not allowed to use any built-in exponent function or operator, such as pow(x, 0.5) or x ** 0.5.

// Constraints:
// 0 <= x <= 2^31 - 1

pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    } else if x == 2 {
        return 1;
    }
    let mut max = i32::min(x / 2 + 1, 46341);
    let mut min = 1;
    let mut current = (max + min) / 4;
    loop {
        let c_sq = current * current;
        if c_sq == x {
            return current;
        } else if c_sq < x {
            min = current;
            current = (max + min) / 2;
        } else {
            max = current;
            current = (max + min) / 2;
        }
        if max - min == 1 {
            return min;
        }
    }
}

#[test]
pub fn t1() {
    println!("{:?}", my_sqrt(2147395600));
}
