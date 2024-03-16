use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [i64; q],
    }

    for x in x.iter() {
        if is_prime(*x) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}

fn is_prime(x: i64) -> bool {
    for i in 2.. {
        if i * i > x { return true }

        if x % i == 0 { return false }
    }

    unreachable!()
}
