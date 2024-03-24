use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let diff = k - (n-1 + n-1);
    if diff >= 0 && diff%2 == 0 {
        println!("Yes");
    } else {
        println!("No")
    }
}
