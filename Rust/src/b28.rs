use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let modulo = 1_000_000_007;
    let fibs = create_fibs(n, modulo);
    println!("{}", fibs[n]);
}

fn create_fibs(n: usize, modulo: usize) -> Vec<usize> {
    let mut fibs = vec![0; n+1];
    fibs[1] = 1;
    fibs[2] = 1;

    for i in 3..=n {
        fibs[i] = (fibs[i-1] + fibs[i-2]) % modulo
    }

    fibs
}
