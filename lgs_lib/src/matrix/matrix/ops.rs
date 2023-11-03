use super::Matrix;
    use std::ops::{self, Index, IndexMut};
    impl ops::Add<Matrix> for Matrix {
        type Output = Matrix;
        fn add(mut self, m: Matrix) -> Self::Output {
            for i in 0..self.height {
                for j in 0..self.width {
                    self[i][j] += m[i][j];
                }
            }
            self
        }
    }

    impl ops::Sub<Matrix> for Matrix {
        type Output = Matrix;
        fn sub(mut self, m: Matrix) -> Self::Output {
            for i in 0..self.height {
                for j in 0..self.width {
                    self[i][j] -= m[i][j];
                }
            }
            self
        }
    }

    impl ops::Mul<f64> for Matrix {
        // todo: run multithreaded
        type Output = Matrix;
        fn mul(mut self, v: f64) -> Self::Output {
            for i in 0..self.height {
                for j in 0..self.width {
                    self[i][j] *= v;
                }
            }
            self
        }
    }

    impl ops::Mul<Matrix> for Matrix {
        type Output = Matrix;
        fn mul(self, m: Matrix) -> Self::Output {
            if self.width != m.height {
                panic!("Matrix dimensions do not match");
            }

            let mut tmp = Matrix::new(self.height, m.width);

            for i in 0..self.height {
                for j in 0..m.width {
                    for k in 0..self.width {
                        tmp[i][j] += self[i][k] * m[k][j];
                    }
                }
            }
            tmp
        }
    }

    impl ops::Div<f64> for Matrix {
        type Output = Matrix;
        fn div(mut self, rhs: f64) -> Self::Output {
            for i in 0..self.height {
                for j in 0..self.width {
                    self[i][j] /= rhs;
                }
            }
            self
        }
    }

    impl PartialEq for Matrix {
        fn eq(&self, other: &Self) -> bool {
            if self.height != other.height || self.width != other.width {
                return false;
            }

            for i in 0..self.height {
                for j in 0..self.width {
                    if self[i][j] != other[i][j] {
                        return false;
                    }
                }
            }
            true
        }

        fn ne(&self, other: &Self) -> bool {
            !self.eq(other)
        }
    }

    impl Index<usize> for Matrix {
        type Output = Vec<f64>;
        fn index(&self, index: usize) -> &Vec<f64> {
            &self.data[index]
        }
    }

    impl IndexMut<usize> for Matrix {
        fn index_mut(&mut self, i: usize) -> &mut Vec<f64> {
            &mut self.data[i]
        }
    }
