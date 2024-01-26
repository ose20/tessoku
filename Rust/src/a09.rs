use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n]
    }

    let imos = calc_imos(h, w, abcd);

    for i in 1..=h {
        for j in 1..=w {
            if j != w { print!{"{} ", imos[i][j]}; }
            else { println!("{}", imos[i][j]); }
        }
    }

}

fn calc_imos(h: usize, w: usize, abcd: Vec<(usize, usize, usize, usize)>) -> Vec<Vec<i32>> {
    let mut imos = vec![vec![0; w+2]; h+2];

    for &(a, b, c, d) in abcd.iter() {
        imos[a][b] += 1;
        imos[a][d+1] -= 1;
        imos[c+1][b] -= 1;
        imos[c+1][d+1] += 1;
    }

    for i in 1..=h {
        for j in 1..=w {
            imos[i][j] += imos[i][j-1];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            imos[i][j] += imos[i-1][j];
        }
    }


    imos
}
