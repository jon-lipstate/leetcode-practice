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
