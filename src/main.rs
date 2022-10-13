use kaprekars_constants::{algo::*, numbers::gen};
use std::{fs, path::Path};

#[tokio::main]
async fn main() {
    let nums: Vec<(Int, Int)> = gen().into_iter().map(|n| (n, count(n))).collect();
    let lines: Vec<String> = nums.iter().map(|n| format!("{},{}", n.0, n.1)).collect();
    let list = lines.join("\n");
    fs::write(Path::new("data").join("nums.csv").as_path(), format!("num,iter\n{}", list)).unwrap()
}
