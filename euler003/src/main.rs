//Largest Prime Factor
//

//The prime factors of 13195 are 5, 7, 13 and 29.

//What is the largest prime factor of the number 600851475143 ?

use std::f64;
//use std::vec::Vec;
//use std::iter;

fn prime (n: u64) -> bool{
    let limit = (n as f64).sqrt() as u64;
    
    for i in 2..=limit {
	if n % i == 0 {
	    return false;
	}
    }
    true
}

fn largest_prime_factor (big_num: u64) -> u64{
    let limit = (big_num as f64).sqrt() as u64;

    let mut current_largest = 0;
    
    for i in 2..=limit {
	if big_num % i == 0 && prime(i){
	    current_largest = i;
	}
    }
    current_largest
}
    
/*fn prime_test () {
    let mut nums: Vec<f64> = Vec::new();

    for i in 1..100 {
	nums.push(i as f64);
    }

    //[1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64, 9_f64, 10_f64];

    let primes = nums.retain(|&x| prime(x));

    println!("{:?}", primes);
    //return primes.iter().map(|x| println!("{}", x.to_string()));
}*/
//factors
//fn euler3() -> u32 {
    
//}

fn main() {
    assert_eq!(largest_prime_factor(13195), 29);
    println!("Project Euler # 3: {}", largest_prime_factor(600851475143));
    //println!("{}", (9.0 % 3.0));
    
    //prime_test();
}
