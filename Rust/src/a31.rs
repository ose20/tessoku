use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    println!("{}", n/3 + n/5 - n/15);
}
