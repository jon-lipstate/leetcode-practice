// Given a m x n matrix grid which is sorted in DESCENDING order both row-wise and column-wise, return the number of negative numbers in grid.
// Follow up: Could you find an O(n + m) solution?

// Constraints:
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 100
// -100 <= grid[i][j] <= 100
pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut neg_vals = 0;
    for row in grid {
        let mut l = 0;
        let mut h = row.len();
        let mut m;
        while l < h {
            m = l + (h - l) / 2;
            if row[m] < 0 {
                h = m;
            } else {
                l = m + 1;
            }
        }
        neg_vals+=row.len() - h;
    }
    neg_vals as i32
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
