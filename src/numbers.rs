use crate::algo::*;

pub fn gen() -> Vec<Int> {
    let mut nums: Vec<Int> = (1000..10000).collect();
    nums.retain(|x| ![1111, 2222, 3333, 4444, 5555, 6666, 7777, 8888, 9999].contains(x));
    nums
}
