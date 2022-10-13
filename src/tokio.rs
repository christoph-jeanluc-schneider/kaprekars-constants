use crate::algo::*;

pub async fn count_all(nums: Vec<Int>, chunk_size: Int) -> Vec<(Int, Int)> {
    let mut all: Vec<(Int, Int)> = vec![];
    let chunks = nums.len() as Int / chunk_size + 1;

    let mut handles = vec![];

    for i in 0..chunks {
        let start = (i * chunk_size) as usize;
        let mut end = start + chunk_size as usize;
        if i + 1 == chunks {
            end = nums.len();
        }
        let chunk = nums[start..end].to_owned();
        handles.push(tokio::spawn(async move {
            chunk.into_iter().map(|n| (n, count(n))).collect()
        }));
    }

    for handle in handles {
        let mut results = handle.await.unwrap();
        all.append(&mut results);
    }

    all
}
