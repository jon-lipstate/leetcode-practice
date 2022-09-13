// Search for a value target in an m x n integer matrix. This matrix has the following properties:
// Integers in each row are sorted from left to right.
// The first integer of each row is greater than the last integer of the previous row.

// Constraints:
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 100
// -10^4 <= matrix[i][j], target <= 10^4
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    unimplemented!()
}

#[test]
fn t1() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(true, search_matrix(matrix, 3));
}
#[test]
fn t2() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(false, search_matrix(matrix, 13));
}
