use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }

    let sum_of_one = s.iter().filter(|c| **c == '1').count();
    if sum_of_one % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
