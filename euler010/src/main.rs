/*Project Euler #10- Summation of Primes

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/
fn is_prime(n: u32) -> bool {
    let limit = (n as f32).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn euler10() -> u64 {
    let mut n = 1;
    let mut prime_sum: u64 = 0;

    while n < 2000000 {
        if is_prime(n) {
            prime_sum += n as u64;
        }
        n += 1;
    }
    prime_sum
}

fn main() {
    println!("Project Euler #10: {}", euler10());
}
