use proconio::input;

fn main() {
    input! {
        n: i32
    }

    println!("{}", format!("{:010b}", n));
}
