// You are climbing a staircase. It takes n steps to reach the top.
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

// Example 1:

// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
// Example 2:

// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step

// Constraints:
// 1 <= n <= 45

// https://www.youtube.com/watch?v=Y0lT9Fck7qI&t=1086s
//Dynamic Programing / Memoization
// calculate from n to zero, retain last two results (since 2 options for stepping), tree of options is sum of prev two items.
pub fn climb_stairs(n: i32) -> i32 {
    let mut a = 1; //n-1
    let mut b = 1; //
    for _ in 0..(n-1) {
        let tmp = a;
        a = a + b;
        b = tmp;
    }
    a
}

#[test]
fn t0() {
    println!("{}", climb_stairs(3));
}
