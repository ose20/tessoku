use proconio::input;

fn main() {
    input! {
        n: i64,        
    }

    solve(n);
}

fn solve(n: i64) {
    // r[i][j] := 0 ~ n までの数の中で、10^i の位に j が出現する回数
    let mut r: Vec<Vec<i64>> = vec![vec![0; 10]; 15];
    let pow10 = calc_pow_table();

    for i in 0..=14 {
        let ith_digit = n / pow10[i] % 10;
        let base = n / pow10[i+1] * pow10[i+1] / 10;
        for j in 0..=9 {
            if j < ith_digit {
                r[i][j as usize] = base + pow10[i];
            } else if j == ith_digit {
                r[i][j as usize] = base + (n % pow10[i]) + 1;
            } else {
                r[i][j as usize] = base;
            }
        }
    }

    let ans = r.iter()
        .flat_map(|vi| vi.iter()
            .enumerate()
            .map(|(j, rij)| (j as i64) * *rij))
        .sum::<i64>();

    println!("{}", ans);
}

fn calc_pow_table() -> Vec<i64> {
    let mut table = vec![1; 16];
    for i in 1..16 {
        table[i] = table[i-1] * 10;
    }

    table
}
