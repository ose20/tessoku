use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", n);
    for i in 1..=n {
        let from = i;
        let to = if i == n { 1 } else { i + 1 };
        println!("{} {}", from, to);
    }
}
