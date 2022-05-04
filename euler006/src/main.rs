/*The sum of the squares of the first ten natural numbers is,
1^2 + 2^2.... = 385
The square of the sum of the first ten natural numbers is,

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is

.

Find the difference between the sum of the squares
of the first one hundred natural numbers and the square of the sum.
 */

fn euler6() -> u32 {
    let mut sum_of_squares = 0;
    for i in 1..=100 {
        sum_of_squares += i * i;
    }

    let mut to_be_squared = 0;
    for i in 1..=100 {
        to_be_squared += i;
    }

    //let squared_sum = to_be_squared * to_be_squared;

    //sum_of_squares - (to_be_squared * to_be_squared)
    //squared_sum
    (to_be_squared * to_be_squared) - sum_of_squares
}

fn main() {
    println!("Project Euler #6 {}", euler6());
}
