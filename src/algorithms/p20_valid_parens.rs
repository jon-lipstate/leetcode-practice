pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    let mut ci = s.chars();
    loop {
        match ci.next() {
            None => {
                break;
            }
            Some(c) => match c {
                '(' => stack.push(Grp::Paren),
                '{' => stack.push(Grp::Curly),
                '[' => stack.push(Grp::Bracket),
                ')' => {
                    if stack.len() == 0 {
                        return false;
                    }
                    if let Some(l) = stack.last() {
                        if l != &Grp::Paren {
                            return false;
                        } else {
                            stack.pop();
                        }
                    }
                }
                '}' => {
                    if stack.len() == 0 {
                        return false;
                    }
                    if let Some(l) = stack.last() {
                        if l != &Grp::Curly {
                            return false;
                        } else {
                            stack.pop();
                        }
                    }
                }
                ']' => {
                    if stack.len() == 0 {
                        return false;
                    }
                    if let Some(l) = stack.last() {
                        if l != &Grp::Bracket {
                            return false;
                        } else {
                            stack.pop();
                        }
                    }
                }
                _ => unreachable!(),
            },
        };
    }
    stack.len() == 0
}

#[derive(PartialEq, Eq, Debug)]
enum Grp {
    Paren,
    Curly,
    Bracket,
}

// Constraints:
// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.

#[test]
fn test() {
    let s1 = "([])".to_string();
    let s2 = "([)]".to_string();
    println!("{},{}", is_valid(s1), is_valid(s2));
}
