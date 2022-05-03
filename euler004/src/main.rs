//Project Euler #4
// Palindrome product of 3 digit numbers

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn is_palindrome(n: u32) -> bool {
    let sn = n.to_string();
    if sn == reverse_string(&sn) {
        return true;
    }
    false
}

fn euler4() -> u32 {
    let mut big_palindrome = 0;

    for i in 100..999 {
        for j in 100..999 {
            if is_palindrome(i * j) && (i * j) > big_palindrome {
                big_palindrome = i * j;
            }
        }
    }
    big_palindrome
}

fn main() {
    println!("Project Euler # 4: {}", euler4());
}
