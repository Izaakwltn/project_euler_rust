/*Project Euler #9
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

struct Triple {
    a: u32,
    b: u32,
    c: u32,
}

fn triple(a: u32, b: u32, c: u32) -> Triple {
    Triple { a, b, c }
}

fn triple_product(t: &Triple) -> u32 {
    t.a * t.b * t.c
}

fn is_pythagorean_triple(t: &Triple) -> bool {
    (t.a * t.a) == (t.b * t.b) + (t.c * t.c)
}

fn euler9() -> u32 {
    for i in 5..999 {
        let limit = 999 - i;
        for j in 1..limit {
            let t = triple(i, j, 1000 - i - j);
            if is_pythagorean_triple(&t) {
                return triple_product(&t);
            }
        }
    }
}

fn main() {
    println!("Project Euler #9: {}", euler9());
}
