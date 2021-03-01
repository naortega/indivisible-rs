use std::env;
use std::collections::VecDeque;

fn main() {
    let args:Vec<String> = env::args().collect();

    // get the first `n` primes
    let n:usize = args[1].parse().unwrap();
    let mut primes:VecDeque<u64> = VecDeque::with_capacity(n);
    // first prime
    println!("{}", 2);
    primes.push_back(2);

    let mut candidate:u64 = 3;

    while primes.len() < n
    {
        let mut i = 0;
        let mut test_prime = primes.get(i).unwrap();
        let mut is_prime = true;
        while *test_prime < (candidate as f64).sqrt() as u64
        {
            if candidate % test_prime == 0
            {
                is_prime = false;
                break;
            }

            i += 1;
            test_prime = primes.get(i).unwrap();
        }

        if is_prime
        {
            println!("{}", candidate);
            primes.push_back(candidate);
        }
        candidate += 2;
    }
}
