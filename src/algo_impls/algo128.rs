pub type Int = i128;
type Iterations = Int;

const K: Int = 6174;
const MAX_ITER: Int = 10;

pub fn count(mut n: Int) -> Iterations {
    for i in 0..MAX_ITER {
        match n {
            K => return i,
            0 => return -1,
            _ => n = next(n),
        };
    }
    -1
}

fn next(n: Int) -> Int {
    let digits = to_digits(n);
    to_num(desc(digits)) - to_num(asc(digits))
}

fn to_digits(n: Int) -> [Int; 4] {
    [n / 1000 % 10, n / 100 % 10, n / 10 % 10, n % 10]
}

fn asc(d: [Int; 4]) -> [Int; 4] {
    let mut arr = d.clone();
    arr.sort();
    arr
}

fn desc(d: [Int; 4]) -> [Int; 4] {
    let mut arr = d.clone();
    arr.sort();
    arr.reverse();
    arr
}

fn to_num(n: [Int; 4]) -> Int {
    n[0] * 1000 + n[1] * 100 + n[2] * 10 + n[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_count() {
        assert_eq!(count(6174), 0);
        assert_eq!(count(7977), 4);
        assert_eq!(count(5317), 1);
        assert_eq!(count(0), -1);
        assert_eq!(count(5555), -1);
    }

    #[ignore]
    #[test]
    fn test_next() {
        assert_eq!(next(5986), 4176);
        assert_eq!(next(4176), 6174);
        assert_eq!(next(6174), 6174);
        assert_eq!(next(5555), 0);
    }

    #[ignore]
    #[test]
    fn test_to_digits() {
        assert_eq!(to_digits(1234), [1, 2, 3, 4]);
        assert_eq!(to_digits(6174), [6, 1, 7, 4]);
        assert_eq!(to_digits(200), [0, 2, 0, 0]);
        assert_eq!(to_digits(0), [0, 0, 0, 0]);
    }

    #[ignore]
    #[test]
    fn test_asc() {
        assert_eq!(asc([6, 1, 7, 4]), [1, 4, 6, 7]);
        assert_eq!(asc([1, 2, 3, 4]), [1, 2, 3, 4]);
    }

    #[ignore]
    #[test]
    fn test_desc() {
        assert_eq!(desc([6, 1, 7, 4]), [7, 6, 4, 1]);
        assert_eq!(desc([1, 2, 3, 4]), [4, 3, 2, 1]);
    }

    #[ignore]
    #[test]
    fn test_to_num() {
        assert_eq!(to_num([4, 7, 1, 6]), 4716);
        assert_eq!(to_num([6, 1, 7, 4]), 6174);
        assert_eq!(to_num([0, 1, 2, 3]), 123);
        assert_eq!(to_num([0, 0, 0, 0]), 0);
    }
}
