use std::cmp;

struct Primes {
    primes: Vec<u64>,
}

impl Primes {
    fn iter(&mut self) -> PrimesIterator {
        PrimesIterator {primes: self, cur_idx: 0}
    }

    fn new() -> Self {
        Primes {primes: vec![2, 3, 5]}
    }
}

struct PrimesIterator<'a> {
    primes: &'a mut Primes,
    cur_idx: usize,
}

impl<'a> Iterator for PrimesIterator<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let idx = self.cur_idx;
        if idx >= self.primes.primes.len() {
            //We need to calculate the next prime
            let next_prime = (self.primes.primes.last().unwrap()+1..).filter(|&n| {
                //Try to divide n by all known primes.
                self.primes.primes.iter().all(|&p| {
                    (n/p)*p != n
                })
            }).next().unwrap();
            self.primes.primes.push(next_prime);
        }
        self.cur_idx += 1;
        Some(self.primes.primes[idx])
    }
}

fn main() {
    //What is the largest prime factor of the number 600851475143 ?
    let mut num = 600851475143;
    let mut primes = Primes::new();
    let mut max_factor = 1;
    while max_factor < num {
        let factor = primes.iter().filter(|&p| (num/p)*p == num).next().unwrap();
        println!("{}", factor);
        max_factor = cmp::max(factor, max_factor);
        num /= factor;
    }

    println!("Max factor is: {}", max_factor);
}
