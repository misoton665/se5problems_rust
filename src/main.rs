extern crate se5problems;

use se5problems::problem::{problem1, problem2};

fn main() {
  let args = std::env::args();

  let running_problem_ = args.skip(1).next().expect("must need more than 1 arguments.");
  let running_problem = &running_problem_[..];
  
  match running_problem {
    "1" => problem1::run_1(),
    "2" => problem2::run_2(),
    _  => println!("Problem not found."),
  }
}
