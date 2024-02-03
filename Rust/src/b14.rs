use proconio::input;

fn main() {
    input! {
        n: usize, 
        k: i64,
        mut a: [i64; n],
    }

    solve(a, k);
}

fn solve(a: Vec<i64>, k: i64) {
    let mid = a.len() / 2;
    let left = subset_sums(&a[..mid]);
    let mut right = subset_sums(&a[mid..]);

    right.sort();

    if left.iter().any(|x| right.binary_search(&(k - x)).is_ok()) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn subset_sums(v: &[i64]) -> Vec<i64> {
    let size = v.len();
    let mut res = Vec::new();

    for bit in 0..(1<<size) {
        let val: i64 = (0..size)
            .map(|i| {
                if 1 & (bit >> i) != 0 { v[i] } else { 0 }
            })
            .sum();

        res.push(val);
    }

    res
}
