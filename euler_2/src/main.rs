struct Fib {
    first: u32,
    second: u32,
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new = self.first + self.second;
        self.first = self.second;
        self.second = new;
        Some(self.first)
    }
}

fn fibs() -> Fib {
    Fib {
        first: 1,
        second: 1,
    }
}

fn main() {
    //By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
    let sum = fibs()
        .take_while(|&n| n < 4_000_000)
        .filter(|n| n % 2 == 0)
        .fold(0, |sum, n| sum + n);
    println!("{}", sum);
}
