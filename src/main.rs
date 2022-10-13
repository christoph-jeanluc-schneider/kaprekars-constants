use kaprekars_constants::{tokio::count_all, numbers::gen};

#[tokio::main]
async fn main() {
    let nums: Vec<u32> = vec![1111, 2222, 1000, 2000];
    let results = count_all(nums, 2).await;
    println!("{:?}", results);
    println!("{}", gen().len());
}
