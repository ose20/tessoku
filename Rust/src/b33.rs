use proconio::input;

fn main() {
    input! {
        n: usize,
        _h: usize,
        _w: usize,
        ab: [(usize, usize); n],
    }

    let nim = ab.iter().fold(0, |acc, (a, b)| acc ^ (a-1) ^ (b-1));
    if nim == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
