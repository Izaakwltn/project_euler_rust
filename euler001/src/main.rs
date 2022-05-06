fn euler1() -> u32 {
    /* sum of multiples of 3 or 5 below 1000*/
    let mut sum = 0;

    for i in 3..999 {
        if ((i % 3) == 0) || ((i % 5) == 0) {
            sum += i;
        }
    }

    sum
}

fn main() {
    println!("Project Euler # 1: {}", euler1());
}
