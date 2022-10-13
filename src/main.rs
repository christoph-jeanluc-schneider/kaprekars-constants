use calc::*;
use digits::*;
use rand::*;

mod calc;
mod digits;

fn main() {
    let mut rng = thread_rng();

    let mut input: i32 = rng.gen_range(1000..9999);
    while invalid_i32(&input) {
        input = rng.gen_range(1000..9999)
    }

    let input: Vec<i32> = [0; 20]
        .iter()
        .map(|_| {
            let mut n: i32 = rng.gen_range(1000..9999);
            while invalid_i32(&n) {
                n = rng.gen_range(1000..9999)
            }
            n
        })
        .collect();

    println!("input: {:?}", input);

    for n in input {
        let calc = Calc::from(n).run();
        println!(
            "{} -> {} in {} iterations",
            n,
            i32::from(calc.state),
            calc.counter
        );
    }    
}
