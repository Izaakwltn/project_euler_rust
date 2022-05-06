//Largest Prime Factor
//

//The prime factors of 13195 are 5, 7, 13 and 29.

//What is the largest prime factor of the number 600851475143 ?

use std::f64;

fn prime(n: u64) -> bool {
    let limit = (n as f64).sqrt() as u64;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn largest_prime_factor(big_num: u64) -> u64 {
    let limit = (big_num as f64).sqrt() as u64;

    let mut current_largest = 0;

    for i in 2..=limit {
        if big_num % i == 0 && prime(i) {
            current_largest = i;
        }
    }
    current_largest
}

fn euler3() -> u64 {
    largest_prime_factor(600851475143)
}

fn main() {
    println!("Project Euler # 3: {}", euler3());
}
