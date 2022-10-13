use calc::*;
use digits::*;
use io::{save, load};
use rand::*;
use std::path::Path;

mod calc;
mod digits;
mod io;

const N: usize = 10;

fn main() {
    let mut rng = thread_rng();

    let results: Vec<CalcResult> = [0; N]
        .iter()
        .map(|_| {
            let mut n: i32 = rng.gen_range(1000..9999);
            while invalid_i32(&n) {
                n = rng.gen_range(1000..9999)
            }
            n
        })
        .map(|n| CalcResult::from(n))
        .collect();

    save(results, Path::new("temp").join("save.file").as_path());

    let loaded = load(Path::new("temp").join("save.file").as_path());
    for calc in loaded.iter() {
        println!("CalcResult {{ num: {}, iter: {} }}", calc.num, calc.iter);
    }
}

