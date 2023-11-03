use structopt::StructOpt;
use lgs_lib::LGS;
use lgs_lib::LGS::Matrix;


/// # Opt
/// These options are used to parse the command line arguments and are required for the program to run.
///
/// ## Options and defaults
///
/// - _**-m \<matrix\>**_: flag to set the matrix to be used for solving an linear system. should be passed in form of **[[1.0],[m.n]]**
/// - _**-b <\vector\>**_: Vector to be solved against. should be passed in form of **[0.0, 0.0]**
/// - _**-d**_: return determinant of the matrix passed as an argument (not implemented yet)
/// - _**-i**_: return the inverse matrix of the matrix passed as an argument (not implemented yet)
/// - _**-t**_: return the transposed matrix passed as an argument (not implemented yet)
#[derive(Debug, StructOpt)]
#[structopt(name = "matrix", about = "Cli tool for matrix operations, including Solving tool for LGS")]
struct Opt {
    #[structopt(short = "m", long = "matrix", default_value = "[[]]")]
    matrix: String,
    #[structopt(short = "b", long = "vector", default_value = "[]")]
    vec: String,
}

///
/// # Main
/// This is the entry point for the program.
///
fn main() {
    let opt = Opt::from_args();
    let matrix = Matrix::from_str(&opt.matrix[..]);
    let binding = opt.vec.replace("[", "").replace("]", "");
    let b:Vec<f64> = binding.split(",").map(|s| s.trim().parse::<f64>().unwrap()).collect();

    let (m,v) = LGS::solve(matrix, b);

    println!("{:?} => {:?}",m,v);
}
