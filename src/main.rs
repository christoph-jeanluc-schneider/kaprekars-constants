use kaprekars_constants::algo::*;
use rand::*;

const N: usize = 10;

fn main() {
    let mut rng = thread_rng();

    let numbers: Vec<Int> = [0; N].iter().map(|_| rng.gen_range(1000..10000)).collect();

    let results: Vec<(Int, Int)> = numbers.into_iter().map(|n| (n, count(n))).collect();

    println!("{:?}", results);
}
