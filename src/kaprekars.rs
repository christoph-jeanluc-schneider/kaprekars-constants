use crate::digits::Digits;

const K: i32 = 6174;

pub struct Iteration {
    pub state: Digits,
    pub count: i32,
    pub is_done: bool,
}

impl From<i32> for Iteration {
    fn from(number: i32) -> Self {
        Iteration {
            state: Digits::from(number),
            count: 0,
            is_done: false,
        }
    }
}

impl Iteration {
    pub fn next(&mut self) -> i32 {
        self.count += 1;
        self.state = iteration(self.state.to_owned());
        let number = i32::from(self.state.to_owned());
        self.is_done = number.eq(&K);
        number
    }
}

pub fn iteration(digits: Digits) -> Digits {
    if i32::from(digits.clone()) == K {
        return Digits::from(K);
    }
    let minuend = i32::from(digits.desc());
    let subtrahend = i32::from(digits.asc());

    println!("{} - {} = {}", minuend, subtrahend, minuend - subtrahend);

    Digits::from(minuend - subtrahend)
}
