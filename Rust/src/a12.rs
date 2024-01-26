use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let ans = solve(a, k);
    println!("{ans}");
}

fn solve(a: Vec<i64>, k: i64) -> i64 {
    let mut ng = 0;
    let mut ok = 1_000_000_000;

    while ng+1 < ok {
        let mid = (ng + ok) / 2;

        let stats: i64 = a.iter().map(|a| &mid / a).sum();
        if stats < k {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    ok
}


