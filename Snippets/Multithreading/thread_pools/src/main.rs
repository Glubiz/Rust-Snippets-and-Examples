use rayon::prelude::*;

fn main() {
    let input = vec![1, 2, 3, 4, 5];
    let output: Vec<_> = input.par_iter().map(|x| x * 2).collect();

    println!("{:?}", output);
}
