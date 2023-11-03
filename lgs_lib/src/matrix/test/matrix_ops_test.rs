use crate::matrix::matrix::*;

#[test]
fn test_matrix_add() {
    let m1 = Matrix::from_data(vec![vec![1_f64, 2_f64], vec![3_f64, 4_f64]]);
    let m2 = Matrix::from_data(vec![vec![5_f64, 6_f64], vec![7_f64, 8_f64]]);
    let expected = Matrix::from_data(vec![vec![6_f64, 8_f64], vec![10_f64, 12_f64]]);

    assert_eq!(m1 + m2, expected);
}

#[test]
fn test_matrix_multiply_by_float() {
    let m1 = Matrix::from_data(vec![vec![1_f64, 2_f64], vec![3_f64, 4_f64]]);
    assert_eq!(m1 * 0.0, Matrix::new(2, 2))
}

#[test]
fn test_matrix_multiply_different_size() {
    let m1 = Matrix::from_data(vec![vec![1_f64, 2_f64]]);
    let m2 = Matrix::from_data(vec![vec![5_f64], vec![7_f64]]);
    let expected = Matrix::from_data(vec![vec![19_f64]]);

    assert_eq!(m1 * m2, expected);
}

#[test]
fn test_matrix_multiply() {
    let m1 = Matrix::from_data(vec![vec![1_f64, 2_f64], vec![3_f64, 4_f64]]);
    let m2 = Matrix::from_data(vec![vec![5_f64, 6_f64], vec![7_f64, 8_f64]]);
    let expected = Matrix::from_data(vec![vec![19_f64, 22_f64], vec![43_f64, 50_f64]]);

    assert_eq!(m1 * m2, expected);
}


#[test]
fn test_matrix_transpose_of_normal() {
    let m = Matrix::from_data(vec![vec![1_f64, 0_f64], vec![0_f64, 1_f64]]);
    let expected = Matrix::from_data(vec![vec![1_f64, 0_f64], vec![0_f64, 1_f64]]);

    assert_eq!(m.transpose(), expected);
}

#[test]
fn test_matrix_transpose() {
    let m = Matrix::from_data(vec![vec![1_f64, 2_f64], vec![3_f64, 4_f64]]);
    let expected = Matrix::from_data(vec![vec![1_f64, 3_f64], vec![2_f64, 4_f64]]);
    let transp = m.transpose();
    assert_eq!(transp, expected)
}



