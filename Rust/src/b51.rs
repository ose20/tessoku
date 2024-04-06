use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = Vec::new();
    for (i, c) in s.iter().enumerate() {
        match c {
            '(' => stack.push(i + 1),
            ')' => {
                println!("{} {}", stack.pop().unwrap(), i + 1);
            }
            _ => unreachable!(),
        }
    }
}
