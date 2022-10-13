use std::{fs, path::Path};

use crate::algo::*;

pub fn _save(results: Vec<(Int, Int)>, path: &Path) {
    let encoded = bincode::serialize(&results).unwrap();
    fs::write(path, encoded).unwrap()
}

pub fn _load(path: &Path) -> Vec<(Int, Int)> {
    let bytes = fs::read(path).unwrap();
    bincode::deserialize(&bytes[..]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = vec![(7977, 4)];
        _save(
            expected.to_owned(),
            Path::new("temp").join("test.bincode").as_path(),
        );
        let actual = _load(Path::new("temp").join("test.bincode").as_path());
        assert_eq!(expected, actual);
    }
}
