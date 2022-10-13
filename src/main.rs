use digits::*;
use kaprekars::*;
use rand::*;

mod digits;
mod kaprekars;

fn main() {
    let mut rng = thread_rng();

    let mut input: i32 = rng.gen_range(1000..9999);
    while invalid_i32(&input) {
        input = rng.gen_range(1000..9999)
    }

    println!("input: {}", input);

    let calc = Calc::from(input).run();
    
    println!(
        "{} -> {} in {} iterations",
        input,
        i32::from(calc.state),
        calc.counter
    );
}
