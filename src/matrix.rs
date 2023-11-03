pub mod Matrix {
    use std::ops::{self, Index, IndexMut};

    #[derive(Debug, Clone)]
    pub(crate) struct Matrix {
        pub height: usize,
        pub width: usize,
        data: Vec<Vec<f64>>,
    }

    impl Matrix {
        pub fn new(height: usize, width: usize) -> Matrix {
            Matrix {
                height,
                width,
                data: vec![vec![0_f64; height]; width],
            }
        }

        pub fn from_str(arr_str: &str) -> Matrix {
            let mut data: Vec<Vec<f64>> = vec![];
            let sub_arrays = arr_str[1..arr_str.len() - 1].as_bytes();
            if sub_arrays.len() <= 2{
                return Matrix {
                    height: 0,
                    width: 0,
                    data,
                };
            }

            let mut j = 0;
            for i in 0..sub_arrays.len() {
                if '[' as u8 == sub_arrays[i] {
                    j = i + 1;
                    continue;
                }

                if ']' as u8 == sub_arrays[i] {
                    let nums = String::from_utf8_lossy(&sub_arrays[j..i]);
                    let nums = nums.split(',').map(|s| s.trim().parse::<f64>().unwrap()).collect();
                    data.push(nums)
                }
            }

            Matrix {
                height: data.len(),
                width: data[0].len(),
                data,
            }
        }

        pub fn from_data(data: Vec<Vec<f64>>) -> Matrix {
            Matrix {
                height: data.len(),
                width: data[0].len(),
                data,
            }
        }

        pub fn add_row(&mut self, j: usize, row: Vec<f64>) {
            for i in 0..self.width {
                self[j][i] += row[i];
            }
        }

        pub fn without_linear_dependencies(self) -> Matrix {
            let mut data: Vec<Vec<f64>> = self.data.clone();

            for i in (0..self.height).rev().step_by(1) {
                for j in 0..i {
                    if is_a_linear_depentent_row(&self[j], &self[i]) {
                        data.remove(i);
                    }
                }
            }

            Matrix::from_data(data)
        }

        pub fn determinant(&self) -> f64 {
            if self.height == 1 {
                return self.data[0][0];
            }

            if self.height == 2 {
                return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
            }
            let mut val: f64 = 0.0;
            let base: f64 = 1.0;
            for i in 0..self.height {
                val += base.powi(i as i32) * self.submatrix(i + 1, 0).determinant();
            }

            val
        }

        fn submatrix(&self, row: usize, col: usize) -> Matrix {
            let mut sub = Matrix::new(self.height - 1, self.width - 1);
            for i in 0..row {
                for j in 0..col {
                    sub.data[i][j] = self.data[row + i % self.height][col + j % self.width];
                }
            }
            sub
        }

        pub fn swap_rows(&mut self, row: usize, other: usize) {
            if row >= self.height || other >= self.height {
                panic!("Row or column index out of bounds");
            }

            self.data.swap(row, other);
        }

        pub fn transpose(&self) -> Matrix {
            let mut tmp = Matrix::new(self.width, self.height);
            for i in 0..self.height {
                for j in 0..self.width {
                    tmp[j][i] = self[i][j];
                }
            }
            tmp
        }
    }

    pub(crate) fn is_a_linear_depentent_row(fst: &Vec<f64>, snd: &Vec<f64>) -> bool {
        if fst.is_empty() || snd.is_empty() {
            return false;
        }

        let a: f64 = snd[0] / fst[0];

        for i in 0..fst.len() {
            if snd[i] - fst[i] * a != 0.0 {
                return false;
            }
        }

        true
    }

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
        fn ne(&self, other: &Self) -> bool {
            !self.eq(other)
        }

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
    }

    impl Index<usize> for Matrix {
        type Output = Vec<f64>;
        fn index(&self, index: usize) -> &Vec<f64> {
            &self.data[index]
        }
    }

    impl IndexMut<usize> for Matrix {
        fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<f64> {
            &mut self.data[i]
        }
    }
}

#[cfg(test)]
mod matrix_test;
