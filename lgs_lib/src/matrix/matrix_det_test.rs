use crate::lgs::Matrix;

fn create_4x4_matrix() -> Matrix {
    Matrix::from_data(vec![
        vec![1.0, 2.0, 52.0, 2.0],
        vec![1.0, 0.0, 1.0, 2.0],
        vec![3.0, 2.0, 12.0, 2.0],
        vec![3.0, 5.0, 3.0, 2.0],
    ])
}
#[test]
fn calc_determinant_of_1x1_matrix() {
    let m = Matrix::from_data(vec![vec![1_f64]]);
    assert_eq!(m.det(), 1_f64);
}

#[test]
fn calc_determinant_of_2x2_matrix() {
    let m = Matrix::from_data(
        vec![
            vec![1_f64, 2_f64],
            vec![3_f64, 4_f64],
        ]);
    assert_eq!(m.det(), -2_f64);
}

#[test]
fn calc_determinant_of_4x4_matrix() {
    let m = create_4x4_matrix();
    assert_eq!(m.det(), 684_f64);
}

#[test]
fn calc_determinant_of_3x3_matrix() {
    let m = Matrix::from_data(
        vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 2.0, 2.0],
            vec![3.0, 3.0, 3.0],
        ]);
    assert_eq!(m.det(), 0.0);
}
