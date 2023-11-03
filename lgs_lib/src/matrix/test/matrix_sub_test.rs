use crate::matrix::matrix::Matrix;

fn create_4x4_matrix() -> Matrix {
    Matrix::from_data(vec![
        vec![1.0, 2.0, 52.0, 2.0],
        vec![1.0, 0.0, 1.0, 2.0],
        vec![3.0, 2.0, 12.0, 2.0],
        vec![3.0, 5.0, 3.0, 2.0],
    ])
}
#[test]
fn sub_matrix() {
    let m = Matrix::from_data(vec![vec![1_f64, 2_f64], vec![3_f64, 4_f64]]);
    let expected = Matrix::from_data(vec![vec![4_f64]]);
    assert_eq!(m.submatrix(0, 0), expected);
}


#[test]
fn sub_matrix_from_4x4() {
    let m = create_4x4_matrix();
    let expected = Matrix::from_data(vec![
        vec![0.0, 1.0, 2.0],
        vec![2.0, 12.0, 2.0],
        vec![5.0, 3.0, 2.0],
    ]);
    assert_eq!(m.submatrix(0, 0), expected);
}

#[test]
fn sub_matrix_from_4x4_col_1() {
    let m = create_4x4_matrix();
    let expected = Matrix::from_data(vec![
        vec![1.0, 1.0, 2.0],
        vec![3.0, 12.0, 2.0],
        vec![3.0, 3.0, 2.0],
    ]);

    assert_eq!(m.submatrix(0, 1), expected);
}

#[test]
fn sub_matrix_from_4x4_col_1_row_1() {
    let m = create_4x4_matrix();

    let expected = Matrix::from_data(vec![
        vec![1.0, 52.0, 2.0],
        vec![3.0, 12.0, 2.0],
        vec![3.0, 3.0, 2.0],
    ]);

    assert_eq!(m.submatrix(1, 1), expected);
}
