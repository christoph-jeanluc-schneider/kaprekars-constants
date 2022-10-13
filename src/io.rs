use std::{fs, path::Path};

use crate::calc::CalcResult;

pub fn _save(results: Vec<CalcResult>, path: &Path) {
    let encoded = bincode::serialize(&results).unwrap();
    fs::write(path, encoded).unwrap()
}

pub fn _load(path: &Path) -> Vec<CalcResult> {
    let bytes = fs::read(path).unwrap();
    bincode::deserialize(&bytes[..]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = vec![CalcResult::from(6174)];
        _save(expected.to_owned(), Path::new("temp").join("test.bincode").as_path());
        let actual = _load(Path::new("temp").join("test.bincode").as_path());
        assert_eq!(expected, actual);
    }
}
