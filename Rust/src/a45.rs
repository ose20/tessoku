use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        c: char,
        a: Chars,
    }

    let to_num = |c: char| -> i32{
        match c {
            'W' => 0,
            'R' => 1,
            'B' => 2,
            _ => unreachable!()
        }
    };

    let score = (a.iter().map(|v| to_num(*v)).sum::<i32>()) % 3;
    if to_num(c) == score {
        println!("Yes");
    } else {
        println!("No");
    }
}
