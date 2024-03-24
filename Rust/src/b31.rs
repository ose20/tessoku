use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let ans = 
        (n / 3)
        + (n / 5)
        + (n / 7)
        - (n / 15)
        - (n / 21)
        - (n / 35)
        + (n / 105);

    println!("{}", ans);
}
