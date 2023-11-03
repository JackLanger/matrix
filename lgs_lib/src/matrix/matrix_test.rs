use crate::matrix::Matrix::*;


#[test]
fn test_create_matrix_from_json_str(){
    
    let m :Matrix = Matrix::from_str("[[2.0,2.0],[3.0,4.0],[5.0,6.0]]");
    let expected = Matrix::from_data(vec![vec![2_f64, 2_f64],vec![3_f64, 4_f64],vec![5_f64, 6_f64]]);
    assert_eq!(m,expected)
}

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

#[test]
fn test_matrix_with_no_linear_dependencies() {
    let m: Matrix = Matrix::from_data(vec![vec![1_f64, 2_f64, 3_f64], vec![4_f64, 4_f64, 6_f64]]);
    let actual: Matrix = m.without_linear_dependencies();
    let expectd: Matrix =
        Matrix::from_data(vec![vec![1_f64, 2_f64, 3_f64], vec![4_f64, 4_f64, 6_f64]]);

    assert_eq!(actual, expectd);
}

#[test]
fn test_row_is_linear_dependant_with_empty_vectors() {
    assert!(!is_a_linear_depentent_row(&vec![], &vec![]));
}

#[test]
fn test_row_linear_is_linear_dependent_is_true(){
    assert!(is_a_linear_depentent_row(&vec![1.0], &vec![2.0]));
    assert!(is_a_linear_depentent_row(&vec![2.0], &vec![1.0]));
}

#[test]
fn test_row_linear_dependent_row_with_3_values(){
        assert!(!is_a_linear_depentent_row(&vec![1_f64,2_f64,3_f64],&vec![3_f64,4_f64,6_f64]));
        assert!(!is_a_linear_depentent_row(&vec![1_f64,2_f64,3_f64],&vec![3_f64,3_f64,3_f64]));
}


#[test]
fn test_remove_linear_dependent_row_from_matrix(){
    
    let m :Matrix = Matrix::from_data(vec![vec![2_f64, 2_f64],vec![3_f64, 4_f64],vec![5_f64, 6_f64], vec![6_f64, 6_f64]]);
    let expected = Matrix::from_data(vec![vec![2_f64, 2_f64],vec![3_f64, 4_f64],vec![5_f64, 6_f64]]);
    assert_eq!(m.without_linear_dependencies(),expected)
}