use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if check(n, &s) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn check(n: usize, s: &Vec<char>) -> bool {
    for i in 2..n {
        if s[i-2] == s[i-1] && s[i-1] == s[i] { return true }
    }

    false
}
