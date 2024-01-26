use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n],
    }


    match a.binary_search(&x) {
        Ok(i) => println!("{}", i+1),
        Err(_) => println!("www"),
    }
}

