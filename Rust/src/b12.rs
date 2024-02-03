use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    solve(n);
}

fn solve(n: f64) {
    let mut left = 0.0;
    let mut right = 110.0;
    
    for _ in 0..100 {
        let mid = (left + right) / 2.0;

        if f(mid) <= n {
            left = mid;
        } else {
            right =mid;
        }
    }

    println!("{:.10}", left)
}


fn f(x: f64) -> f64 {
    x*x*x + x
}
