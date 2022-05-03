/* Project Euler # 5


2520 is the smallest number that can be divided by each of the numbers
from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

*/

fn twenty_divisible(n: u32) -> bool {
    for i in 1..=20 {
        if n % i != 0 {
            return false;
        }
    }
    true
}

fn peuler5() -> u32 {
    let mut n = 1;

    loop {
        if twenty_divisible(n) {
            return n;
        }
        n += 1;
    }
}

fn main() {
    println!("Project Euler #5: {}", peuler5());
}
