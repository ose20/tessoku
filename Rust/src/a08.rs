use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i32; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q]
    }

    let cum = calc_accum(h, w, &x);

    for &(a, b, c, d) in abcd.iter() {
        let ans = cum[c][d] - cum[a-1][d] - cum[c][b-1] + cum[a-1][b-1];
        println!("{ans}");
    }
}

fn calc_accum(h: usize, w: usize, x: & Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sum = vec![vec![0; w+1]; h+1];

    for i in 1..=h {
        for j in 1..=w {
            sum[i][j] = x[i-1][j-1];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            sum[i][j] += sum[i][j-1];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            sum[i][j] += sum[i-1][j]
        }
    }

    sum
}
