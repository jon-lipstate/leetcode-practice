// Given a characters array letters that is sorted in non-decreasing order and a character target, return the smallest character in the array that is larger than target.
// Note that the letters wrap around.
// For example, if target == 'z' and letters == ['a', 'b'], the answer is 'a'.

// Constraints:
// 2 <= letters.length <= 10^4
// letters[i] is a lowercase English letter.
// letters is sorted in non-decreasing order.
// letters contains at least two different characters.
// target is a lowercase English letter.

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut lu = letters.clone();
    lu.dedup();
    let l = lu.len() - 1;
    if target < lu[0] {
        return lu[0];
    } else if target >= lu[l] {
        return lu[0];
    }
    let mut min = 0;
    let mut max = l;
    while max > min {
        let mid = min + (max - min) / 2;
        match lu[mid].cmp(&target) {
            std::cmp::Ordering::Equal => {
                if mid + 1 <= l {
                    return lu[mid + 1];
                } else {
                    return lu[0];
                }
            }
            std::cmp::Ordering::Greater => {
                max = mid;
            }
            std::cmp::Ordering::Less => {
                min = mid + 1;
            }
        }
    }

    lu[min]
}

#[test]
fn t0() {
    let letters = ['c', 'f', 'j'].to_vec();
    assert_eq!('c', next_greatest_letter(letters, 'a'));
}
#[test]
fn t1() {
    let letters = ['c', 'f', 'j'].to_vec();
    assert_eq!('f', next_greatest_letter(letters, 'c'));
}
#[test]
fn t2() {
    let letters = ['c', 'f', 'j'].to_vec();
    assert_eq!('f', next_greatest_letter(letters, 'd'));
}
#[test]
fn tz() {
    let letters = ['c', 'f', 'j'].to_vec();
    assert_eq!('c', next_greatest_letter(letters, 'z'));
}
#[test]
fn tj() {
    let letters = ['c', 'f', 'j'].to_vec();
    assert_eq!('c', next_greatest_letter(letters, 'j'));
}
#[test]
fn te() {
    let letters = ['e', 'e', 'e', 'e', 'e', 'e', 'n', 'n', 'n', 'n'].to_vec();
    assert_eq!('n', next_greatest_letter(letters, 'e'));
}
