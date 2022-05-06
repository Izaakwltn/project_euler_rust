//use std::ops::Not;

fn euler2() -> u128 {
    /* ?Find the sum of all even fibonacci numbers under 4 million*/
    let mut current_fib = 1;
    let mut last_fib = 1;
    let mut temp;

    let mut fibsum = 0;

    while current_fib < 4000000 {
        temp = last_fib; //1 1
        last_fib = current_fib; //1 2
        current_fib = last_fib + temp; //2 3

        //println!("{}", current_fib);

        if (current_fib % 2) == 0 {
            fibsum += current_fib;
        }
    }
    fibsum
}

fn main() {
    let fs = euler2().to_string();
    println!("Project Euler #2: {}", fs);
}
