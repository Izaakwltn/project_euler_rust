//Euler # 2

pub fn euler2() -> u32 {
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
