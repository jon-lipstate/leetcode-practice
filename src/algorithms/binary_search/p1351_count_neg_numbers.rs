// Given a m x n matrix grid which is sorted in DESCENDING order both row-wise and column-wise, return the number of negative numbers in grid.
// Follow up: Could you find an O(n + m) solution?

// Constraints:
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 100
// -100 <= grid[i][j] <= 100
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    unimplemented!()
}

#[test]
fn t1() {
    // Explanation: There are 8 negatives number in the matrix.
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    let result = count_negatives(grid);
    assert_eq!(8, result);
}
#[test]
fn t2() {
    // Explanation: There are 8 negatives number in the matrix.
    let grid = vec![vec![3, 2], vec![1, 0]];

    let result = count_negatives(grid);
    assert_eq!(0, result);
}
