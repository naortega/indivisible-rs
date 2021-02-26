use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    // get the first `n` primes
    let n:u64 = args[1].parse().unwrap();
    // first prime
    println!("{}", 2);
    // already have first prime
    let mut prime_count:u64 = 1;
    let mut candidate:u64 = 3;

    while prime_count < n
    {
        let mut aux:u64 = 3;
        let mut is_prime = true;
        while aux < (candidate as f64).sqrt() as u64
        {
            if candidate % aux == 0
            {
                is_prime = false;
                break;
            }
            aux += 2;
        }

        if is_prime
        {
            println!("{}", candidate);
            prime_count += 1;
        }
        candidate += 2;
    }
}
