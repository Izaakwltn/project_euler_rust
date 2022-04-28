/*fn euler1() -> u32 {
    /* sum of multiples of 3 or 5 below 1000*/
    let mut sum = 0;

    for i in 3..999 {
	if ((i % 3) == 0) | ((i % 5) == 0) {
	    sum += i;
	}
    }

    return sum;
    
}*/


fn euler2() -> u32 {
    /* ?Find the sum of all even fibonacci numbers under 4 million*/
    let mut fib = 1;
    let mut last_fib = 1;
    let mut temp;

    let mut fibsum = 0;
    
    while fib < 100 {
	if fib % 2 == 0 {
	    fibsum += fib;
	    temp = fib;
	    fib += last_fib;
	    last_fib = temp;
	}
    }
    return fibsum;
	
}

fn main() {
    //eventually have prompt where you give a number and it returns your project euler answer
    //let e1 = euler1();
    println!("Euler 1: {}", euler1().to_string());

    println!("Euler 2: {}", euler2().to_string());

}
