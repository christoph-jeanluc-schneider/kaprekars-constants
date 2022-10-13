use kaprekars_constants::{digits::*, algo::*, calc::*};
use rand::*;

const N: usize = 10;

fn main() {
    let mut rng = thread_rng();

    let numbers_a: Vec<i32> = [0; N]
        .iter()
        .map(|_| {
            let mut n: i32 = rng.gen_range(1000..9999);
            while invalid_i32(&n) {
                n = rng.gen_range(1000..9999)
            }
            n
        })
        .collect();

    let numbers_b: Vec<Int> = numbers_a.iter().map(|n| *n as Int).collect();

    let results_a: Vec<CalcResult> = numbers_a.into_iter().map(|n| CalcResult::from(n)).collect();

    let results_b: Vec<(Int, Int)> = numbers_b.into_iter().map(|n| (n, count(n))).collect();

    println!("{:?}", results_a);
    println!("{:?}", results_b);
}
