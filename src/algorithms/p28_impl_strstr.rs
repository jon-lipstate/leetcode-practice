pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle == "" {
        return 0;
    } else if haystack == "" {
        return -1;
    } else if needle.len() > haystack.len() {
        return -1;
    }

    let h_chars: Vec<char> = haystack.chars().collect();
    let n_chars: Vec<char> = needle.chars().collect();
    let mut hi = 0;
    let mut ni = 0;
    let mut first_matched: i32 = -1;
    loop {
        let h = h_chars[hi];
        let n = n_chars[ni];
        // println!("{h},{n}");
        if h == n {
            if first_matched < 0 {
                // println!("set first_matched @ {hi}");
                first_matched = hi as i32;
            }
            hi += 1;
            ni += 1;
        } else {
            if first_matched > -1 {
                hi = first_matched as usize + 1;
                ni = 0;
                first_matched = -1;
                // println!("reset first_matched @ {hi}");
            } else {
                hi += 1;
            }
        }
        //test for end of lists:
        if ni >= n_chars.len() {
            return first_matched;
        } else if hi >= h_chars.len() {
            return -1;
        }
    }
}

#[test]
fn test3() {
    let haystack = "mississippi".to_string();
    let needle = "issip".to_string();
    let n = str_str(haystack, needle);
    println!("{n}");
}

#[test]
fn test() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    let n = str_str(haystack, needle);
    println!("{n}");
}

#[test]
fn test2() {
    let haystack = "a".to_string();
    let needle = "a".to_string();
    let n = str_str(haystack, needle);
    println!("{n}");
}
