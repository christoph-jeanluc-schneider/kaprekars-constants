use crate::digits::*;
use serde::*;

pub const K: i32 = 6174;

#[derive(Debug, Clone, PartialEq)]
pub struct Calc {
    pub input: Digits,
    pub state: Digits,
    pub counter: i32,
    pub is_done: bool,
}

impl From<i32> for Calc {
    fn from(number: i32) -> Self {
        let digits = Digits::from(number);
        Calc {
            input: digits.to_owned(),
            state: digits,
            counter: 0,
            is_done: false,
        }
    }
}

impl Calc {
    pub fn next(&mut self) -> i32 {
        self.counter += 1;
        self.state = next(&self.state);
        let number = i32::from(self.state.to_owned());
        self.is_done = number.eq(&K);
        number
    }

    pub fn run(mut self) -> Self {
        for _ in 0..100 {
            self.next();
            if self.is_done {
                break;
            }
        }
        self
    }
}

pub fn next(digits: &Digits) -> Digits {
    if i32::from(digits.clone()) == K {
        return Digits::from(K);
    }
    Digits::from(i32::from(digits.desc()) - i32::from(digits.asc()))
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CalcResult {
    pub num: i32,
    pub iter: i32,
}

impl From<i32> for CalcResult {
    fn from(input: i32) -> Self {
        let calc = Calc::from(input).run();
        CalcResult {
            num: input,
            iter: calc.counter,
        }
    }
}
