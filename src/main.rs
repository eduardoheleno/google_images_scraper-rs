mod args;

use std::env;
use args::Args;

fn main() {
    let vec_args: Vec<String> = env::args().collect();
    let args = Args::new(vec_args);
}
