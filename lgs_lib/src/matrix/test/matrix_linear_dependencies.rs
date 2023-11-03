use crate::matrix::matrix::{is_a_linear_dependent_row, Matrix};
#[test]
fn test_matrix_with_no_linear_dependencies() {
    let m: Matrix = Matrix::from_data(vec![vec![1_f64, 2_f64, 3_f64], vec![4_f64, 4_f64, 6_f64]]);
    let actual: Matrix = m.remove_linear_dependent_rows();
    let expectd: Matrix =
            Matrix::from_data(vec![vec![1_f64, 2_f64, 3_f64], vec![4_f64, 4_f64, 6_f64]]);

    assert_eq!(actual, expectd);
}

#[test]
fn test_row_is_linear_dependant_with_empty_vectors() {
    assert!(!is_a_linear_dependent_row(&vec![], &vec![]));
}

#[test]
fn test_row_linear_is_linear_dependent_is_true() {
    assert!(is_a_linear_dependent_row(&vec![1.0], &vec![2.0]));
    assert!(is_a_linear_dependent_row(&vec![2.0], &vec![1.0]));
}

#[test]
fn test_row_linear_dependent_row_with_3_values() {
    assert!(!is_a_linear_dependent_row(&vec![1_f64, 2_f64, 3_f64], &vec![3_f64, 4_f64, 6_f64]));
    assert!(!is_a_linear_dependent_row(&vec![1_f64, 2_f64, 3_f64], &vec![3_f64, 3_f64, 3_f64]));
}


#[test]
fn test_remove_linear_dependent_row_from_matrix() {
    let m: Matrix = Matrix::from_data(vec![vec![2_f64, 2_f64], vec![3_f64, 4_f64], vec![5_f64, 6_f64], vec![6_f64, 6_f64]]);
    let expected = Matrix::from_data(vec![vec![2_f64, 2_f64], vec![3_f64, 4_f64], vec![5_f64, 6_f64]]);
    assert_eq!(m.remove_linear_dependent_rows(), expected)
}