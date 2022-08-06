// Given two binary strings a and b, return their sum as a binary string.

// Constraints:
// 1 <= a.length, b.length <= 104
// a and b consist only of '0' or '1' characters.
// Each string does not contain leading zeros except for the zero itself.
pub fn add_binary(a: String, b: String) -> String {
    let mut ai = a.chars().rev();
    let mut bi = b.chars().rev();
    let max_len = usize::max(a.len(), b.len());
    let mut s = String::new();
    let mut carry = 0;
    for _ in 0..max_len {
        let aval = match ai.next().unwrap_or('0') {
            '0' => 0,
            '1' => 1,
            _ => unreachable!(),
        };
        let bval = match bi.next().unwrap_or('0') {
            '0' => 0,
            '1' => 1,
            _ => unreachable!(),
        };
        let current_value = aval ^ bval ^ carry;
        carry = (aval & bval) | (aval & carry) | (bval & carry);
        if current_value == 0 {
            s.push('0');
        } else {
            s.push('1');
        }
        // println!("a:{aval},b:{bval},carry:{carry},a^b^c: {current_value}");
    }
    if carry == 1 {
        s.push('1');
    }
    return s.chars().rev().collect();
}

fn i32_to_bin_str(n: i32) -> String {
    let mut val = n;
    let mut s = String::new();
    if val == 0 {
        return "0".to_string();
    }
    while val > 0 {
        if val % 2 == 1 {
            s.push('1');
        } else {
            s.push('0');
        }
        val /= 2;
    }

    return s.chars().rev().collect();
}
fn str_to_bin(a: &str) -> i32 {
    let mut ac = a.chars().rev();
    let mut ab = 0b0;
    let mut i = 0;
    while let Some(c) = ac.next() {
        match c {
            '0' => {}
            '1' => ab += i32::pow(2, i),
            _ => unreachable!(),
        };
        i += 1;
    }

    return ab;
}

#[test]
fn tt() {
    // let s = "10110";
    // let i = str_to_bin(s);
    let i = 22;
    let ss = i32_to_bin_str(i);
    println!("{i}, {ss}");
}

#[test]
fn t0() {
    let a = "11".to_string();
    let b = "1".to_string();
    assert_eq!("100".to_string(), add_binary(a, b));
}

#[test]
fn t1() {
    let a = "1010".to_string();
    let b = "1011".to_string();
    assert_eq!("10101".to_string(), add_binary(a, b));
}

#[test]
fn test() {
    let a = "0".to_string();
    let b = "0".to_string();
    assert_eq!("0".to_string(), add_binary(a, b));
}

#[test]
fn tes22t() {
    let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
    let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
    assert_eq!("110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string(), add_binary(a, b));
}
