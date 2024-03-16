use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let nim = a.iter().fold(0, |acc, x| acc ^ x);
    if nim == 0 {
        println!("Second");
    } else {
        print!("First");
    }
}
