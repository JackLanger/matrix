use std::process::exit;

use structopt::StructOpt;
use lgs_lib::lgs::lgs;
use lgs_lib::matrix::matrix::Matrix;


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
#[structopt(name = "matrix", about = "Cli tool for matrix operations, including Solving tool for LGS")]
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
    let matrix = Matrix::from_str(&opt.matrix[..]);
    let binding = opt.vec.replace("[", "").replace("]", "");

    println!("M:{:?}", matrix.get_data());

    if opt.determinant {
        println!("Det:{:?}", matrix.det());
    }

    // if opt.inverse {
    //     let inv = matrix.inverse();
    //     println!("Inverse: {:?}", inv);
    // }

    if opt.transpose {
        println!("T: {:?}", matrix.transpose().get_data());
    }

    if opt.solve {
        if opt.vec.is_empty() || opt.vec == "[]" {
            eprintln!("Invalid or empty Vector provided");
            exit(1);
        } else {
            let b: Vec<f64> = binding.split(",").map(|s| s.trim().parse::<f64>().unwrap()).collect();
            let (m, v) = lgs::solve(matrix, b);
            println!("M:{:?}, b: {:?}", m.get_data(), v);
        }
    }
}
