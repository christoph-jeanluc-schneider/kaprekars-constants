use crate::digits::Digits;

const K: i32 = 6174;

pub struct Calc {
    pub state: Digits,
    pub counter: i32,
    pub is_done: bool,
}

impl From<i32> for Calc {
    fn from(number: i32) -> Self {
        Calc {
            state: Digits::from(number),
            counter: 0,
            is_done: false,
        }
    }
}

impl Calc {
    pub fn next(&mut self) -> i32 {
        self.counter += 1;
        self.state = next(self.state.to_owned());
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

pub fn next(digits: Digits) -> Digits {
    if i32::from(digits.clone()) == K {
        return Digits::from(K);
    }
    let minuend = i32::from(digits.desc());
    let subtrahend = i32::from(digits.asc());

    println!("{} - {} = {}", minuend, subtrahend, minuend - subtrahend);

    Digits::from(minuend - subtrahend)
}
