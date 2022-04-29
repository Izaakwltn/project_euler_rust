//Largest Prime Factor
//

//The prime factors of 13195 are 5, 7, 13 and 29.

//What is the largest prime factor of the number 600851475143 ?

use std::f64;
use std::vec::Vec;
//use std::iter;

fn prime (n: f64) -> bool{
    let mut i = 2.0;
    
    loop {
	i += 1.0;
	
	if  (n % i) == 0.0 {
	    return false;
	}
	if i > n.sqrt() {
	    return true;
	}
    }
}

fn prime_test () {
    let mut nums: Vec<f64> = Vec::new();

    for i in 1..100 {
	nums.push(i as f64);
    }

    //[1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64, 9_f64, 10_f64];

    let primes = nums.retain(|&x| prime(x));

    println!("{:?}", primes);
    //return primes.iter().map(|x| println!("{}", x.to_string()));
}
//factors
//fn euler3() -> u32 {
    
//}

fn main() {
    println!("Hello, world!");
    assert_eq! (prime(3.0), true);
    assert_eq! (prime(4.0), false);
    assert_eq! (prime(5.0), true);
    prime_test();
}
