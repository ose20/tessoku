use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [f64; n],
    }

    let ans = solve(k, &a)
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", ans);
}

fn solve(k: usize, a: &[f64]) -> Vec<usize> {
    // 「票数 / 議席数」の値で二分探索
    // この値を v としたとき、　政党 i の獲得議席数は、 a[i]/v の切り捨て
    // a[i]/議席 >= v を満たす整数議席の数なので、
    // 議席 <= a[i]/v と変形されるので切り捨て

    let mut left = 1.0;
    let mut right = 1_000_000_000.0;
    for _ in 0..50 {
        let v = (left + right) / 2.0;
        let sheets = a
            .iter()
            .map(|ai| (ai / v).floor() as usize)
            .collect::<Vec<_>>();

        let sum = sheets.iter().sum::<usize>();
        match sum.cmp(&k) {
            std::cmp::Ordering::Equal => return sheets,
            std::cmp::Ordering::Less => {
                right = v;
            }
            std::cmp::Ordering::Greater => {
                left = v;
            }
        }
    }

    unreachable!()
}
