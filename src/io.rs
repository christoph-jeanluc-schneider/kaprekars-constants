use std::{fs, path::Path};

use crate::calc::CalcResult;

pub fn save(results: Vec<CalcResult>, path: &Path) {
    let encoded = bincode::serialize(&results).unwrap();
    fs::write(path, encoded).unwrap()
}

pub fn load(path: &Path) -> Vec<CalcResult> {
    let bytes = fs::read(path).unwrap();
    bincode::deserialize(&bytes[..]).unwrap()
}
