// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".
// Constraints:
// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut common_depth: usize = strs.iter().map(|s| s.len()).min().unwrap(); // max length is the shortest string
    let mut common = &strs[0][0..common_depth]; // substring of first string

    for s in &strs {
        while common != &s[0..common_depth] {
            common_depth -= 1;
            common = &common[0..common_depth];
            if common_depth == 0 {
                return String::default();
            }
        }
    }
    common.to_string()
}

#[test]
fn t() {
    let t1 = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]; //expect: fl
    let t2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]; // expect ""
    println!("t1 {}", longest_common_prefix(t1));
    println!("t2 {}", longest_common_prefix(t2));
}
