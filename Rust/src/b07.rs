use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let imos = calc_imos(t, lr);

    for i in 0..t { println!("{}", imos[i]) }
}

fn calc_imos(t: usize, lr: Vec<(usize, usize)>) -> Vec<i32> {
    let mut imos = vec![0; t+1];

    for &(l, r) in lr.iter() {
        imos[l] += 1;
        imos[r] -= 1;
    }

    for i in 1..=t {
        imos[i] += imos[i-1];
    }

    imos
}
