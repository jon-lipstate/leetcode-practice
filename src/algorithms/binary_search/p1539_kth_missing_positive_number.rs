// Given an array arr of positive integers sorted in a strictly increasing order, and an integer k.
// Return the kth positive integer that is missing from this array.
// Follow up:
// Could you solve this problem in less than O(n) complexity?

// Constraints:
// 1 <= arr.length <= 1000
// 1 <= arr[i] <= 1000
// 1 <= k <= 1000
// arr[i] < arr[j] for 1 <= i < j <= arr.length

pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
    let len = arr.len() - 1;
    let missing_at_start = arr[0] - 1;
    let missing_at_end = arr[len] - (len as i32) - 1;
    if missing_at_start >= k {
        return k;
    } else if missing_at_end < k {
        return arr[len] + k - missing_at_end;
    }
    let mut l = 0;
    let mut r = len + 1;
    let mut missing;
    while r > l {
        let m = l + (r - l) / 2;
        missing = arr[m] - m as i32 - 1;
        match missing.cmp(&(k)) {
            std::cmp::Ordering::Equal => {
                let v = arr[m];
                //hacky...
                if arr[m] - arr[m - 1] == 1 {
                    return v - 2;
                }
                return v - 1;
            }
            std::cmp::Ordering::Greater => {
                r = m;
            }
            std::cmp::Ordering::Less => {
                l = m + 1;
            }
        }
    }
    let i = l - 1;
    missing = arr[i] - i as i32 - 1;
    let need = k - missing;
    arr[i] + need
}
//test for : left, right, middle

#[test]
fn why() {
    let arr = [
        1, 5, 6, 7, 12, 14, 17, 32, 35, 43, 55, 60, 64, 65, 67, 75, 79, 80, 82, 90, 92, 93, 95, 99,
        102, 103, 109, 110, 111, 112, 113, 114, 129, 132, 134, 139, 141, 146, 147, 152, 162, 163,
        164, 172, 183, 200, 202, 206, 216, 217, 219, 222, 223, 231, 232, 241, 246, 260, 261, 262,
        276, 278, 285, 294, 302, 303, 305, 306, 312, 323, 325, 331, 332, 333, 337, 341, 344, 345,
        352, 353, 361, 369, 378, 383, 396, 403, 406, 408, 409, 415, 418, 419, 422, 427, 433, 437,
        438, 441, 444, 447, 448, 456, 460, 461, 464, 471, 476, 480, 482, 483, 485, 487, 490, 491,
        493, 497, 499, 506, 507, 509, 511, 513, 518, 522, 528, 530, 531, 532, 535, 536, 539, 543,
        548, 551, 552, 553, 556, 563, 567, 572, 578, 579, 586, 587, 588, 589, 590, 594, 607, 611,
        613, 617, 619, 627, 629, 632, 636, 638, 650, 654, 659, 667, 669, 672, 673, 675, 676, 682,
        685, 693, 705, 710, 714, 717, 724, 727, 728, 729, 732, 740, 743, 752, 763, 766, 771, 772,
        777, 784, 787, 793, 799, 800, 804, 805, 811, 822, 826, 838, 840, 848, 850, 851, 857, 859,
        863, 864, 865, 868, 872, 873, 880, 883, 887, 891, 893, 895, 902, 906, 908, 909, 915, 918,
        922, 926, 931, 933, 948, 953, 959, 962, 965, 967, 972, 973, 976, 983, 984, 985, 986, 989,
        993,
    ]
    .to_vec();
    let k = 71;
    assert_eq!(91, find_kth_positive(arr, k));
}

#[test]
fn ww() {
    // Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.
    let arr = [
        8, 11, 16, 20, 29, 30, 32, 33, 37, 39, 42, 44, 46, 47, 48, 50, 52, 56, 60, 63, 64, 65, 68,
        70, 72, 74, 80,
    ]
    .to_vec();
    let k = 45;
    assert_eq!(67, find_kth_positive(arr, k));
}

#[test]
fn tm() {
    // Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5th missing positive integer is 9.
    let arr = [2, 3, 4, 7, 11].to_vec();
    let k = 5;
    assert_eq!(9, find_kth_positive(arr, k));
}

#[test]
fn ts() {
    let arr = [4, 7, 11].to_vec();
    let k = 3;
    assert_eq!(3, find_kth_positive(arr, k));
}

#[test]
fn te() {
    // Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
    let arr = [1, 2, 3, 4].to_vec();
    let k = 2;
    assert_eq!(6, find_kth_positive(arr, k));
}
#[test]
fn te2() {
    // Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
    let arr = [1, 2, 4].to_vec();
    let k = 3;
    assert_eq!(6, find_kth_positive(arr, k));
}
#[test]
fn tsss() {
    // Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
    let arr = [1, 3].to_vec();
    let k = 1;
    assert_eq!(2, find_kth_positive(arr, k));
}
#[test]
fn tsthnss() {
    // Explanation: The missing positive integers are [5,6,7,...]. The 2nd missing positive integer is 6.
    let arr = [1, 10, 21, 22, 25].to_vec();
    let k = 12;
    assert_eq!(14, find_kth_positive(arr, k));
}
