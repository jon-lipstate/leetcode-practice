#![allow(non_snake_case)]
// We are playing the Guess Game. The game is as follows:
// I pick a number from 1 to n. You have to guess which number I picked.
// Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
// You call a pre-defined API int guess(int num), which returns three possible results:
// -1: Your guess is higher than the number I picked (i.e. num > pick).
// 1: Your guess is lower than the number I picked (i.e. num < pick).
// 0: your guess is equal to the number I picked (i.e. num == pick).
// Return the number that I picked.

// Constraints:
// 1 <= n <= 2^31 - 1
// 1 <= pick <= n

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */
static mut VAL: i32 = 6;
unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&VAL) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
    }
}

unsafe fn guessNumber(n: i32) -> i32 {
    let mut low = 0;
    let mut high = n;
    loop {
        let mid = (high - low) / 2 + low;
        match guess(mid) {
            0 => {
                return mid;
            }
            1 => {
                low = mid + 1;
            }
            -1 => {
                high = mid - 1;
            }
            _ => {
                unreachable!();
            }
        };
    }
}

#[test]
fn t0() {
    unsafe {
        VAL = 6;
        let n = 10;
        let r = guessNumber(n);
        assert_eq!(VAL, r);
    }
}

#[test]
fn t1() {
    unsafe {
        VAL = 1;
        let n = 1;
        let r = guessNumber(n);
        assert_eq!(VAL, r);
    }
}

#[test]
fn t2() {
    unsafe {
        VAL = 1;
        let n = 2;
        let r = guessNumber(n);
        assert_eq!(VAL, r);
    }
}

#[test]
fn t3() {
    unsafe {
        VAL = 1702766719;
        let n = 2126753390;
        let r = guessNumber(n);
        assert_eq!(VAL, r);
    }
}
