use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();
    a.reverse();

    println!("{}", a[0] + a[1]);
}
