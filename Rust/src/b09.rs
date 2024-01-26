use proconio::input;

static MAXV: usize = 1500;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let imos = calc_imos(abcd);
    let ans = solve(imos);
    println!("{ans}");

}

fn calc_imos(abcd: Vec<(usize, usize, usize, usize)>) -> Vec<Vec<i32>> {
    let mut imos = vec![vec![0; MAXV+5]; MAXV+5];

    for &(a, b, c, d) in abcd.iter() {
        imos[a+1][b+1] += 1;
        imos[a+1][d+1] -= 1;
        imos[c+1][b+1] -= 1;
        imos[c+1][d+1] += 1;
    }

    for i in 0..=MAXV {
        for j in 1..=MAXV {
            imos[i][j] += imos[i][j-1];
        }
    }

    for j in 0..=MAXV {
        for i in 1..=MAXV {
            imos[i][j] += imos[i-1][j];
        }
    }

    imos
}

fn solve(imos: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    for i in 1..=MAXV {
        for j in 1..=MAXV {
            if imos[i][j] > 0 { ans += 1; }
        }
    }

    ans
}
