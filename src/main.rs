use std::process::exit;

use lgs_lib::lgs::lgs;
use lgs_lib::matrix::matrix::Matrix;
use structopt::StructOpt;

/// # Opt
/// These options are used to parse the command line arguments and are required for the program to run.
///
/// ## Options and defaults
///
/// - _**-m \<matrix\>**_: flag to set the matrix to be used for solving an linear system. should be passed in form of **[[1.0],[m.n]]**
/// - _**-b <\vector\>**_: Vector to be solved against. should be passed in form of **[0.0, 0.0]**
/// - _**-s**_: solve the equation for the given vector.
/// - _**-a**_: approximate the solution for the given vector (not implemented yet).
/// - _**-d**_: return determinant of the matrix passed as an argument
/// - _**-i**_: return the inverse matrix of the matrix passed as an argument (not implemented yet)
/// - _**-t**_: return the transposed matrix passed as an argument
#[derive(Debug, StructOpt)]
#[structopt(
    name = "matrix",
    about = "Cli tool for matrix operations, including Solving tool for LGS"
)]
struct Opt {
    #[structopt(short = "m", long = "matrix", default_value = "")]
    matrix: String,
    #[structopt(short = "b", long = "vector", default_value = "")]
    vec: String,
    #[structopt(short = "d", long = "determinant")]
    determinant: bool,
    #[structopt(short = "i", long = "inverse")]
    inverse: bool,
    #[structopt(short = "t", long = "transpose")]
    transpose: bool,
    #[structopt(short = "s", long = "solve")]
    solve: bool,
#[structopt(short = "a", long = "aproximate")]
    aproximate: bool,
}

///
/// # Main
/// This is the entry point for the program.
///
fn main() {
    let opt = Opt::from_args();
    if opt.matrix.is_empty() || opt.matrix == "[[]]" {
        eprintln!("No matrix provided");
        exit(1);
    }
    let mut matrix = Matrix::from_str(&opt.matrix[..]);
    
    println!("M:{:?}", matrix.get_data());
    
    calc_determinant_if_opt(&opt,&matrix);
    calculate_inverse_if_opt(&opt, &matrix);
    transpose_if_opt(&opt,&matrix);
    solve_if_opt(opt,  matrix);
}

fn solve_if_opt(opt: Opt, matrix: Matrix) {
    
    let binding = opt.vec.replace("[", "").replace("]", "");
    if opt.solve {
        if opt.vec.is_empty() || opt.vec == "[]" {
            eprintln!("Invalid or empty Vector provided");
            exit(1);
        } else {
            let b: Vec<f64> = binding
                .split(",")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();

                let m = if opt.aproximate {
                    if matrix.width != b.len() {
                        println!("Matrix cannot be aproximated, as the width of the matrix does not match the length of the vector, 
                        squaring the matrix will result in a square matrix that is of different dimensions than the vector."); 
                
                        exit(1);
                    }
                
                    let clone = matrix.clone();
                    (clone.transpose()) * clone
                } else {
                    matrix.clone()
                };
                
            let (m, v) = lgs::solve(m, b);
            println!("M:{:?}, b: {:?}", m.get_data(), v);
        }
    }
}


fn transpose_if_opt(opt: &Opt, matrix: &Matrix) {
    if opt.transpose {
        println!("T: {:?}", matrix.transpose().get_data());
    }
}

fn calculate_inverse_if_opt(opt: &Opt, matrix: &Matrix) {
    if opt.inverse {
        let inv = lgs::inverse(matrix.clone());
        println!("Inverse: {:?}", inv);
    }
}

fn calc_determinant_if_opt(opt: &Opt, matrix: &Matrix) {
    if opt.determinant {
        println!("Det:{:?}", matrix.det());
    }
}
