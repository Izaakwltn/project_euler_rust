/* Project Euler #7

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10,001st prime number?

*/

fn is_prime(n: u64) -> bool {
    let limit = (n as f64).sqrt() as u64;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn euler7() -> u64 {
    let mut prime_count = 0;
    let mut n = 1;

    while prime_count < 10001 {
        n += 1;
        if is_prime(n) {
            prime_count += 1;
        }
    }
    n
}

fn main() {
    println!("Project Euler #7 {}", euler7());
}
