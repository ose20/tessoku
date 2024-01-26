use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i32; n],
    }

    let ans = solve(a, k);
    println!("{ans}");
}

fn solve(a: Vec<i32>, k: i64) -> i64 {
    let mut ans = 0;

    let mut right = 0;
    for left in 0..(a.len()) {
        if right <= left { right = left+1; }
        
        while right < a.len() && (a[right] - a[left]) as i64 <= k {
            right += 1;
        }

        ans += (right - left - 1) as i64;
    }

    ans
}
