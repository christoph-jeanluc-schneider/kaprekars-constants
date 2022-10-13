use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Digits(pub Vec<i32>);

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<i32> for Digits {
    fn from(number: i32) -> Self {
        let mut digits: Vec<i32> = Vec::new();
        if number < 1000 {
            digits.push(0);
        }

        let mut remaining = number;
        while remaining > 0 {
            let val = remaining % 10;
            digits.push(val);
            remaining /= 10;
        }

        digits.reverse();
        Digits(digits)
    }
}

impl From<Digits> for i32 {
    fn from(digits: Digits) -> Self {
        let mut number: i32 = 0;
        let mut base: i32 = 10i32.pow(i32::max(digits.0.len() as i32 - 1, 0) as u32);
        for d in digits.0.iter() {
            number += d * base;
            base /= 10;
        }
        number
    }
}

impl Digits {
    /// returns sorted copy
    pub fn asc(&self) -> Self {
        let mut digits = self.clone();
        digits.0.sort_unstable();
        digits
    }

    /// returns sorted copy
    pub fn desc(&self) -> Self {
        let mut digits = self.clone();
        digits.0.sort_unstable();
        digits.0.reverse();
        digits
    }
}

const INVALID: [i32; 10] = [0, 1111, 2222, 3333, 4444, 5555, 6666, 7777, 8888, 9999];

pub fn invalid_i32(number: &i32) -> bool {
    INVALID.contains(number)
}
