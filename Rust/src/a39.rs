use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    println!("{}", solve(n, &mut lr));
}

fn solve(_n: usize, lr: &mut Vec<(usize, usize)>) -> usize {
    lr.sort_by_key(|k| k.1);
    let mut ans = 0;
    let mut maxend = 0;

    for (l, r) in lr.iter() {
        if maxend <= *l {
            ans += 1;
            maxend = *r
        }
    }

    ans
}
