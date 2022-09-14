// Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.

// Constraints:
// 0 <= c <= 2^31 - 1

//hint: int^2 in rng [0,c−a^2]
pub fn judge_square_sum(c: i32) -> bool {
    unimplemented!()
}

#[test]
fn t1() {
    assert_eq!(true, judge_square_sum(5));
}
#[test]
fn t2() {
    assert_eq!(false, judge_square_sum(3));
}
