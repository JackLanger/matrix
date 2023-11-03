pub mod lgs;
pub mod matrix;

use std::process;
use structopt::StructOpt;
use crate::lgs::LGS;
use crate::matrix::Matrix::Matrix;

#[derive(Debug, StructOpt)]
#[structopt(name = "matrix", about = "Cli tool for matrix operations, including Solving tool for LGS")]
struct Opt {
    #[structopt(short = "m", long = "matrix", default_value = "[[]]")]
    matrix: String,
    #[structopt(short = "b", long = "vector", default_value = "[]")]
    vec: String,
}

fn main() {
    let opt = Opt::from_args();
    let matrix = Matrix::from_str(&opt.matrix[..]);
    let binding = opt.vec.replace("[", "").replace("]", "");
    let b:Vec<f64> = binding.split(",").map(|s| s.trim().parse::<f64>().unwrap()).collect();

    let (m,v) = LGS::solve(matrix, b);

    println!("{:?} => {:?}",m,v);
}
