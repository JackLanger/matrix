/// # Description:
/// Linear Gaussian Separation (LGS),
/// is a method for solving a system of linear equations against a given set of variables.
pub mod lgs {
    pub use crate::matrix::matrix::*;


    /// # Caclulate the inverse of a matrix.
    /// An inverse of a matrix is the matrix that results in a normal matrix when multiplied with the matrix. 
    /// A normal matrix is a matrix where all values but the main diagonal are 0.
    /// The main diagonal holds only values that are 1.0;
    /// 
    /// # Parameters:
    /// - m: Matrix - The matrix
    pub fn inverse(m: Matrix) -> Matrix {
        let mut m = m.remove_linear_dependent_rows();
        if m.width != m.height {
            panic!("Matrix must be square");
        }
        
        let mut v = Matrix::new(m.height, m.height);
        for i in 0..m.height {
         v[i][i] = 1.0;
        }

         // shadow as mutable
        // iterate down
        for i in 0..m.height {
            for j in (i + 1)..m.height {
            
                let alpha: f64 =  m[j][i]/ m[i][i];
                let mut row = m[i].clone();
                let mut v_row = v[j].clone();

                for i in 0..row.len(){
                    row[i] *= -alpha;
                    v_row[i] *= -alpha;
                }
    
                m.add_to_row(j, row);
                v.add_to_row(j, v_row);
            }
        }
        // iterate up
        for i in (0..m.height).rev().step_by(1) {
            for j in 0..i {
                let alpha: f64 = m[j][i] / m[i][i];
                let row = m[i].iter().map(|f| f * -alpha).collect();
                let v_row = v[i].iter().map(|f| f * -alpha).collect();
                m.add_to_row(j, row);
                v.add_to_row(j, v_row);
            }
        }
        // normalize
        for i in 0..v.height {
            let a = 1.0/m[i][i]; 
            for k in 0..m.width {
                if v[i][k] == 0.0 {continue;}
                if v[i][k] == -0.0 {m[i][k] = 0.0; continue;}

                v[i][k] *= a;
            }
        }
        v
    }

    /// # Solve
    ///
    /// Solves a system of linear equations. If the Matrix contains linear dependencies,
    /// they will be removed from the matrix and the system will be solved.
    ///
    /// If the system cannot be solved, an error will be returned.
    ///
    /// # Parameters
    ///
    /// m : Matrix the coefficients of the system
    /// v: `Vec<f64>` vector to solve against
    ///
    /// # Panics if
    ///
    /// Panics if the matrix provided is not of the size of the vector or square.
    pub fn solve(m: Matrix, v: Vec<f64>) -> (Matrix, Vec<f64>) {
        let mut m = m.remove_linear_dependent_rows();
        let mut v = v; // shadow as mutable
        
        // let mut m = m;
        if v.len() != m.height {
            panic!("Matrix and vector lengths do not match");
        }

        if m.width != m.height {
            panic!("Matrix must be square");
        }

        // iterate down
        for i in 0..m.height {
            calculate_sub_matrix_down(i, &mut m, &mut v);
        }
        // iterate up
        for i in (0..m.height).rev().step_by(1) {
            calculate_submatrix_up(i, &mut m, &mut v);
        }
        // normalize
        normalize(&mut v, &mut m);
        (m, v)
    }

    fn normalize(v: &mut Vec<f64>, m: &mut Matrix) {
        // let mut result = vec![0.0; v.len()];
        for i in 0..v.len() {
            let a = 1.0/m[i][i]; 
            for k in 0..m.width {
                if m[i][k] == 0.0 {continue;}
                if m[i][k] == -0.0 {m[i][k] = 0.0; continue;}

                m[i][k] *= a;
            }
            v[i] *= a;
        }
    }

    fn calculate_submatrix_up(i: usize, m: &mut Matrix, v: &mut Vec<f64>) {
        for j in 0..i {
            let alpha: f64 = m[j][i] / m[i][i];
            v[j] -= alpha * v[i];
            let row = m[i].iter().map(|f| f * -alpha).collect();
            m.add_to_row(j, row);
        }
    }

    fn calculate_sub_matrix_down(i: usize, m: &mut Matrix, v: &mut Vec<f64>) {
        for j in (i + 1)..m.height {
            
            let alpha: f64 =  m[j][i]/ m[i][i];
            v[j] -= alpha * v[i];

            let mut row = m[i].clone();
            for i in 0..row.len(){
                row[i] *= -alpha;
            }

            m.add_to_row(j, row);
        }
    }


    


}

#[cfg(test)]
mod lgs_test;
mod lgs_inverse_test;
