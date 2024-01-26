use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if (a..=b).any(|x| 100 % x == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
