pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut p = s.chars().peekable();
    while p.peek().is_some() {
        let current = to_int(&p.next().unwrap()).unwrap();
        if let Some(next_char) = p.peek() {
            let next = to_int(next_char).unwrap();
            if next > current {
                sum -= current;
            } else {
                sum += current;
            }
        } else {
            sum += current;
        }
    }

    return sum;
}

fn to_int(c: &char) -> Result<i32, ()> {
    match c {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err(()),
    }
}

#[test]
fn t() {
    // assert_eq!(3, roman_to_int("III".to_string()));
    // assert_eq!(58, roman_to_int("LVIII".to_string()));
    assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
}
