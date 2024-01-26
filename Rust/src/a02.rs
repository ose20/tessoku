use proconio::input;

fn main() {
    input!{
        n: usize,
        x: i32,
        a: [i32; n]
    }

    if a.iter().any(|elt| elt == &x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
