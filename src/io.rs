use std::{fs, path::Path};

use serde::{de::Deserialize, Serialize};

pub fn save<T>(data: &T, path: &Path)
where
    T: ?Sized + Serialize,
{
    let encoded = bincode::serialize(data).unwrap();
    fs::write(path, encoded).unwrap()
}

pub fn load<T>(path: &Path) -> T
where
    for<'a> T: Deserialize<'a>,
{
    let bytes = fs::read(path).unwrap();
    bincode::deserialize(&bytes[..]).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected: Vec<(i16, i16)> = vec![(7977, 4)];
        save(&expected, Path::new("temp").join("test.bincode").as_path());
        let actual: Vec<(i16, i16)> = load(Path::new("temp").join("test.bincode").as_path());
        assert_eq!(expected, actual);
    }
}
