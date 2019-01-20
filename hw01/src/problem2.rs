/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // Check Matrix are compatible to perform multiplication
    let columns = mat1[0].len();
    let rows = mat2.len();
    assert_eq!(columns, rows);
    
    let mut new_matrix = vec![vec![0.; mat2[0].len()]; mat1.len()];
    for i in 0..mat1.len() {
        for j in 0..mat2[0].len() {
            for k in 0..mat1[0].len() {
                new_matrix[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }
    new_matrix
}