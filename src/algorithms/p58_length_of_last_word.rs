// Given a string s consisting of words and spaces, return the length of the last word in the string.

// A word is a maximal substring consisting of non-space characters only.

// Constraints:

// 1 <= s.length <= 104
// s consists of only English letters and spaces ' '.
// There will be at least one word in s.
pub fn length_of_last_word(s: String) -> i32 {
    //trivial solution:
    // let substr = s.trim_end().split_ascii_whitespace().last();
    // return substr.unwrap().len() as i32;
    //iter sol:
    let mut c = s.chars().rev();
    let mut i = 0;
    loop {
        match c.next() {
            None => {
                return i;
            }
            Some(cval) => {
                if cval != ' ' {
                    i += 1
                } else if i > 0 && cval == ' ' {
                    return i;
                }
            }
        }
    }
}

#[test]
fn test1() {
    let s = "Hello World".to_string();
    let l = length_of_last_word(s);
    assert_eq!(l, 5);
}
#[test]
fn test0() {
    let s = "a".to_string();
    let l = length_of_last_word(s);
    assert_eq!(l, 1);
}
#[test]
fn test2() {
    let s = "   fly me   to   the moon  ".to_string();
    let l = length_of_last_word(s);
    assert_eq!(l, 4);
}

#[test]
fn test3() {
    let s = "luffy is still joyboy".to_string();
    let l = length_of_last_word(s);
    assert_eq!(l, 6);
}
