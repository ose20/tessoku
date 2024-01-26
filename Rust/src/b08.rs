use proconio::input;

static MAXV: usize = 1501;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q]
    }

    let cum = calc_cum(xy);

    for &(a, b, c, d) in abcd.iter() {
        let ans = cum[c][d] - cum[c][b-1] - cum[a-1][d] + cum[a-1][b-1];
        println!("{ans}");
    }
}

fn calc_cum(xy: Vec<(usize, usize)>) -> Vec<Vec<i32>> {
    let mut cum = vec![vec![0; MAXV+1]; MAXV+1];

    for &(x, y) in xy.iter() {
        cum[x][y] += 1;
    }

    for i in 0..=MAXV {
        for j in 1..=MAXV {
            cum[i][j] += cum[i][j-1]
        }
    }

    for j in 0..=MAXV {
        for i in 1..=MAXV {
            cum[i][j] += cum[i-1][j];
        }
    }

    cum
}
