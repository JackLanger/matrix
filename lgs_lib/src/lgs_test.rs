use crate::LGS::{Matrix, solve};

#[test]
fn test_solve_with_x() {
    let m: Matrix = Matrix::from_data(vec![vec![1_f64]]);
    let v: Vec<f64> = vec![1_f64];
    let (m, res) = solve(m, v);
    assert_eq!(res, vec![1.0]);
    assert_eq!(m, Matrix::from_data(vec![vec![1_f64]]));
}

#[test]
fn test_solve_valid_lgs() {
    let m: Matrix = Matrix::from_data(vec![vec![2.0, -1.0], vec![4.0, 1.0]]);
    let v: Vec<f64> = vec![6.0, 6.0];
    let (m, res) = solve(m, v);
    assert_eq!(res, vec![2.0,-2.0]);
    assert_eq!(m, Matrix::from_data(vec![vec![1.0, 0.0], vec![0.0, 1.0]]));
}
