/// # Matrix
/// The matrix module contains the implementation and definition of the Matrix struct.
pub mod matrix {
    use std::ops::{self, Index, IndexMut};

    /// Matrix
    /// Matrix struct, providing a simple interface to interact with matrices of floating point numbers.
    #[derive(Debug, Clone)]
    pub struct Matrix {
        pub height: usize,
        pub width: usize,
        data: Vec<Vec<f64>>,
    }

    impl Matrix {
        ///# Description:
        /// Create a new matrix of the given height and width.
        /// The values are initialized to 0.0.
        ///
        /// # Arguments:
        /// - height : usize, the height of the matrix
        /// - width : usize, the width of the matrix
        pub fn new(height: usize, width: usize) -> Matrix {
            Matrix {
                height,
                width,
                data: vec![vec![0.0; height]; width],
            }
        }

        ///# Description:
        /// Create a new matrix from the given string.
        /// If the string does not adhere to the legal format of an JSON array
        /// and/or contains any other values than floating point numbers,
        /// an empty matrix is returned.
        ///
        /// # Arguments:
        /// - arr_str : &str; The string to convert to a matrix.
        pub fn from_str(arr_str: &str) -> Matrix {
            let mut data: Vec<Vec<f64>> = vec![];
            let sub_arrays = arr_str[1..arr_str.len() - 1].as_bytes();
            if sub_arrays.len() <= 2 {
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

        ///# Description:
        /// Create a new matrix from a vector of vectors.
        ///
        /// # Arguments:
        /// - data: `Vec<Vec<f64>>`, The vector of vectors to convert to a matrix.
        pub fn from_data(data: Vec<Vec<f64>>) -> Matrix {
            Matrix {
                height: data.len(),
                width: data[0].len(),
                data,
            }
        }

        ///# Description:
        /// Adds a Vector to a row in the matrix.
        /// # Panics:
        /// Panics if the row is not of the same length as the matrix width.
        pub fn add_to_row(&mut self, j: usize, row: Vec<f64>) {
            if row.len()!= self.width {
                panic!("Row length does not match matrix width");
            }
            for i in 0..self.width {
                self[j][i] += row[i];
            }
        }

        /// # Description:
        /// Remove linear dependencies from the matrix.
        /// This returns a new Matrix without linear dependent rows.
        /// The original matrix is unchanged.
        pub fn remove_linear_dependent_rows(self) -> Matrix {
            let mut data: Vec<Vec<f64>> = self.data.clone();

            for i in (0..self.height).rev().step_by(1) {
                for j in 0..i {
                    if is_a_linear_dependent_row(&self[j], &self[i]) {
                        data.remove(i);
                    }
                }
            }

            Matrix::from_data(data)
        }

        ///# Description:
        /// Calculates the determinant of the matrix.
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

        /// # Description:
        /// create a new matrix that is the submatrix of the current matrix,
        /// beginning at the given row and column.
        fn submatrix(&self, row: usize, col: usize) -> Matrix {
            let mut sub = Matrix::new(self.height - 1, self.width - 1);
            for i in 0..row {
                for j in 0..col {
                    sub.data[i][j] = self.data[row + i % self.height][col + j % self.width];
                }
            }
            sub
        }

        ///# Description:
        /// Swap two rows in the matrix.
        pub fn swap_rows(&mut self, row: usize, other: usize) {
            if row >= self.height || other >= self.height {
                panic!("Row or column index out of bounds");
            }

            self.data.swap(row, other);
        }


        /// # Description:
        /// Create a new matrix that is the transposed of the current matrix.
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

    /// # Description:
    /// Check if a row is linear dependent of another row.
    /// A linear dependent row is a row,
    /// that contains all the same values as the first row,
    /// multiplied by a factor n where n is an element of R.
    ///
    /// e.g. [1, 1, 1] and [2,2,2] where [2, 2, 2] is linear dependent of [1, 1, 1].
    pub(crate) fn is_a_linear_dependent_row(fst: &Vec<f64>, snd: &Vec<f64>) -> bool {
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
}

#[cfg(test)]
mod matrix_test;
