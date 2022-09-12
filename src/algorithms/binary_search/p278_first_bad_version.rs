// You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.

// Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.

// You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.

// Constraints:
// 1 <= bad <= n <= 23^1 - 1

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
struct IsBad {
    first_bad: i32,
}
impl IsBad {
    #![allow(non_snake_case)]
    pub fn isBadVersion(&self, v: i32) -> bool {
        self.first_bad <= v
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut h = n;
        while h > l {
            let mid = l + (h - l) / 2;
            match self.isBadVersion(mid) {
                true => {
                    h = mid;
                }
                false => {
                    l = mid + 1;
                }
            }
        }
        l
    }
}

#[test]
fn t0() {
    let ib = IsBad { first_bad: 4 };
    let n = 5;
    let r = ib.first_bad_version(n);
    assert_eq!(ib.first_bad, r);
}

#[test]
fn t1() {
    let ib = IsBad { first_bad: 1 };
    let n = 1;
    let r = ib.first_bad_version(n);
    assert_eq!(ib.first_bad, r);
}

#[test]
fn t2() {
    let ib = IsBad {
        first_bad: 2_147_483_647,
    };
    let n = 2_147_483_647;
    let r = ib.first_bad_version(n);
    assert_eq!(ib.first_bad, r);
}
