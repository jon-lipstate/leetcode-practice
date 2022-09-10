//
// Given two integer arrays arr1 and arr2, and the integer d, return the distance value between the two arrays.
// The distance value is defined as the ***number of elements arr1[i]*** such that there is not any element arr2[j] where |arr1[i]-arr2[j]| <= d.

// Constraints:
// 1 <= arr1.length, arr2.length <= 500
// -1000 <= arr1[i], arr2[j] <= 1000
// 0 <= d <= 100

pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
    let mut n = 0;
    let mut a2 = arr2.clone();
    a2.sort();
    for i in 0..arr1.len() {
        let c = arr1[i];
        let idx2 = bin_srch(&a2, c);
        let s = i32::abs(c - a2[idx2]) > d;
        let m1 = (idx2 > 0 && i32::abs(c - a2[idx2 - 1]) > d) || idx2 == 0; // prob dont need this check
        let p1 = (idx2 < a2.len() - 1 && i32::abs(c - a2[idx2 + 1]) > d) || idx2 == a2.len() - 1;
        if s && m1 && p1 {
            n += 1;
        }
    }
    n
}

fn bin_srch(arr: &Vec<i32>, target: i32) -> usize {
    let mut r = arr.len() - 1;
    let mut l = 0;

    if arr[r] < target {
        return r;
    } else if arr[0] > target {
        return 0;
    }

    while r > l {
        let m = l + (r - l) / 2;
        match target.cmp(&arr[m]) {
            std::cmp::Ordering::Equal => {
                return m;
            }
            std::cmp::Ordering::Less => {
                r = m;
            }
            std::cmp::Ordering::Greater => {
                l = m + 1;
            }
        }
    }

    l
}

#[test]
fn t1() {
    let a1 = [4, 5, 8].to_vec();
    let a2 = [10, 9, 1, 8].to_vec();
    let d = 2;
    let result = find_the_distance_value(a1, a2, d);
    assert_eq!(2, result);

    // Explanation:
    // For arr1[0]=4 we have:
    // |4-10|=6 > d=2
    // |4-9|=5 > d=2
    // |4-1|=3 > d=2
    // |4-8|=4 > d=2

    // For arr1[1]=5 we have:
    // |5-10|=5 > d=2
    // |5-9|=4 > d=2
    // |5-1|=4 > d=2
    // |5-8|=3 > d=2

    // For arr1[2]=8 we have:
    // |8-10|=2 <= d=2
    // |8-9|=1 <= d=2
    // |8-1|=7 > d=2
    // |8-8|=0 <= d=2
}

#[test]
fn t2() {
    let a1 = [1, 4, 2, 3].to_vec();
    let a2 = [-4, -3, 6, 10, 20, 30].to_vec();
    let d = 3;
    let result = find_the_distance_value(a1, a2, d);
    assert_eq!(2, result);
}

#[test]
fn t3() {
    let a1 = [2, 1, 100, 3].to_vec();
    let a2 = [-5, -2, 10, -3, 7].to_vec();
    let d = 6;
    let result = find_the_distance_value(a1, a2, d);
    assert_eq!(1, result);
}
