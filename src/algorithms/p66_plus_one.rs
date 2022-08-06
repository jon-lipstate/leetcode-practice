// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
// Increment the large integer by one and return the resulting array of digits.

// Constraints:

// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// digits does not contain any leading 0's.

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let len = digits.len();
    if digits.last().unwrap() != &9i32 {
        digits[len - 1] = digits[len - 1] + 1;
        return digits;
    } else {
        let mut carry = 1;
        while len - carry > 0 && digits[len - carry] == 9 {
            carry += 1;
        }
        let mut push_zero = false;
        // println!("{}", digits[len - carry]);
        digits[len - carry] = if digits[len - carry] == 9 {
            push_zero = true;
            1
        } else {
            digits[len - carry] + 1
        };

        for x in len - carry + 1..len {
            // println!("x:{x}, l:{}, c:{}", len, carry);
            digits[x] = 0;
        }
        if push_zero {
            digits.push(0);
        }

        return digits;
    }
}

#[test]
fn t0() {
    let digits = vec![1, 2, 3];
    let expect = [1, 2, 4];
    let p1 = plus_one(digits);
    println!("{:?} sb {:?}", p1, expect);
}

#[test]
fn t1() {
    let digits = vec![4, 3, 2, 1];
    let expect = [4, 3, 2, 2];
    let p1 = plus_one(digits);
    println!("{:?} sb {:?}", p1, expect);
}

#[test]
fn t2() {
    let digits = vec![9];
    let expect = [1, 0];
    let p1 = plus_one(digits);
    println!("{:?} sb {:?}", p1, expect);
}
#[test]
fn t3() {
    let digits = vec![9, 9, 9];
    let expect = [1, 0, 0, 0];
    let p1 = plus_one(digits);
    println!("{:?} sb {:?}", p1, expect);
}
#[test]
fn t4() {
    let digits = vec![8, 9, 9, 9];
    let expect = [9, 0, 0, 0];
    let p1 = plus_one(digits);
    println!("{:?} sb {:?}", p1, expect);
}
