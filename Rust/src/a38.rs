use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    }

    let ans = solve(d, &lrh);
    println!("{}", ans);
}

fn solve(d: usize, lrh: & Vec<(usize, usize, usize)>) -> usize {
    let mut ans = 0;
    for i in 1..=d {
        ans += lrh.iter()
            .filter_map(|(l, r, d)| {
                if *l <= i && i <= *r { Some(d) }
                else { None }
            })
            .min().unwrap_or(&24);
    }

    ans
}