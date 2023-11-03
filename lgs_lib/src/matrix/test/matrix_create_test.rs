use crate::matrix::matrix::Matrix;

#[test]
fn test_create_matrix_from_json_str() {
    let m: Matrix = Matrix::from_str("[[2.0,2.0],[3.0,4.0],[5.0,6.0]]");
    let expected = Matrix::from_data(vec![vec![2_f64, 2_f64], vec![3_f64, 4_f64], vec![5_f64, 6_f64]]);
    assert_eq!(m, expected)
}

#[test]
fn test_create_matrix_from_number_str() {
    let m: Matrix = Matrix::from_str("1.0");
    let expected = Matrix::new(0,0);
    assert_eq!(m, expected)
}


#[test]
fn test_init_empty_matrix() {
    let m = Matrix::new(0, 0);
    assert_eq!(m.height, 0);
    assert_eq!(m.width, 0);
}

#[test]
fn test_init_empty_1x1_matrix() {
    let m = Matrix::new(1, 1);
    assert_eq!(m.height, 1);
    assert_eq!(m.width, 1);
}

#[test]
fn test_init_matrix_with_data() {
    let m = Matrix::from_data(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ]);

    assert_eq!(m.height, 3);
    assert_eq!(m.width, 3);
    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][1], 2.0);
    assert_eq!(m[0][2], 3.0);
    assert_eq!(m[1][0], 4.0);
    assert_eq!(m[1][1], 5.0);
    assert_eq!(m[1][2], 6.0);
    assert_eq!(m[2][0], 7.0);
    assert_eq!(m[2][1], 8.0);
    assert_eq!(m[2][2], 9.0);
}