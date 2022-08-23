// Given an integer n, return true if it is a power of four. Otherwise, return false.
// An integer n is a power of four, if there exists an integer x such that n == 4^x.

// Input: n = 16 -> true
// Input: n = 5 -> false
// Input: n = 1 -> true
// Output: true

// Constraints:
// -2^31 <= n <= 2^31 - 1

pub fn is_power_of_four(n: i32) -> bool {
    if n == 1 || n == 4 {
        //1, 4
        return true;
    }
    if n < 1 || n % 4 != 0 {
        return false;
    }
    //2^0=1; 2^2=4;2^4=16;2^6=64;2^8=256;
    //4^0=1; 4^1=4;4^2=16;4^3=64;4^4=256;
    let mut x = n;
    while x > 1 {
        x = x >> 2;
        //any bit-shifted value must also be mod4
        if x % 4 != 0 {
            return false; //e.g. 68>>2=17, 72>>2=18
        }
        println!("{:?}", x);
        if x == 4 {
            return true;
        }
    }
    false
}
#[test]
fn t0() {
    // println!("{},{}",);
    println!("RESULT: {:?} ", is_power_of_four(72));
}
