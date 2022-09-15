// You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing civilians). The soldiers are positioned in front of the civilians. That is, all the 1's will appear to the left of all the 0's in each row.

// A row i is weaker than a row j if one of the following is true:

// The number of soldiers in row i is less than the number of soldiers in row j.
// Both rows have the same number of soldiers and i < j.
// Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.

// Constraints:
// m == mat.length
// n == mat[i].length
// 2 <= n, m <= 100
// 1 <= k <= m
// matrix[i][j] is either 0 or 1.

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    //n.log(m)
    let rows: Vec<usize> = mat.iter().map(|arr| count_ones(&arr)).collect();
    println!("r: {:?}", rows);
    //should use priority queue but havent learned yet
    let mut v = Vec::new();
    for i in 0..rows.len() {
        v.push((i, rows[i]));
    }
    v.sort_by(|i, j| i.1.cmp(&j.1));
    println!("v: {:?}", v);

    v.into_iter().take(k as usize).map(|i| i.0 as i32).collect()
}
///Count of ones
fn count_ones(mat: &Vec<i32>) -> usize {
    if mat[0] == 0 {
        return 0;
    } else if mat[mat.len() - 1] == 1 {
        return mat.len();
    }
    let mut l = 0;
    let mut r = mat.len() - 1;
    while l < r {
        let m = l + (r - l) / 2;
        let v = mat[m];
        let v1 = mat[m + 1];
        if v == 1 && v1 == 0 {
            return m + 1;
        } else if v == 0 {
            r = m;
        } else {
            l = m + 1;
        }
    }
    unreachable!()
}

#[test]
fn t1() {
    let mat = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 1, 1, 1],
    ];
    let k = 3;
    // Explanation:
    // The number of soldiers in each row is:
    // - Row 0: 2
    // - Row 1: 4
    // - Row 2: 1
    // - Row 3: 2
    // - Row 4: 5
    // The rows ordered from weakest to strongest are [2,0,3,1,4].
    assert_eq!(vec![2, 0, 3], k_weakest_rows(mat, k));
}
#[test]
fn t2() {
    let mat = vec![
        vec![1, 0, 0, 0],
        vec![1, 1, 1, 1],
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
    ];
    let k = 2;
    // Explanation:
    // The number of soldiers in each row is:
    // - Row 0: 1
    // - Row 1: 4
    // - Row 2: 1
    // - Row 3: 1
    // The rows ordered from weakest to strongest are [0,2,3,1].
    assert_eq!(vec![0, 2], k_weakest_rows(mat, k))
}
#[test]
fn t3() {
    let mat = vec![vec![1, 0], vec![0, 0], vec![1, 0]];
    let k = 2;

    assert_eq!(vec![1, 0], k_weakest_rows(mat, k))
}
