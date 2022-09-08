// Given a positive integer num, write a function which returns True if num is a perfect square else False.
// Follow up: Do not use any built-in library function such as sqrt.

// Constraints:
// 1 <= num <= 2^31 - 1

pub fn is_perfect_square(num: i32) -> bool {
    if num == 1 {
        return true;
    }
    let mut low = 1;
    let mut high = i32::min(num / 2, 46340) + 1; //+1 to capture max sq ; 2,147,483,647 sqrt : 46340.95
    while high > low {
        let mid = (low + high) / 2;
        match num.cmp(&(mid * mid)) {
            std::cmp::Ordering::Equal => {
                return true;
            }
            std::cmp::Ordering::Greater => {
                low = mid + 1;
            }
            std::cmp::Ordering::Less => {
                high = mid;
            }
        }
    }

    false
}
#[test]
fn tw() {
    assert_eq!(true, is_perfect_square(4));
}
#[test]
fn tt() {
    assert_eq!(true, is_perfect_square(2_147_395_600));
}
#[test]
fn t0() {
    assert_eq!(true, is_perfect_square(1));
}

#[test]
fn t1() {
    assert_eq!(false, is_perfect_square(2));
}
#[test]
fn t3() {
    assert_eq!(true, is_perfect_square(16));
}
#[test]
fn t4() {
    assert_eq!(false, is_perfect_square(14));
}
