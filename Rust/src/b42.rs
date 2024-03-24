use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64,i64); n],
    }

    let ans = solve(n, ab);
    println!("{}", ans);
}

fn solve(_n: usize, ab: Vec<(i64, i64)>) -> i64 {
    // |x| = max{x, -x} を利用して、4パターンを全探索

    let plusplus: i64 = filter_fold(|(a, b)| a + b > 0, &ab);
    let plusminus: i64 = filter_fold(|(a, b)| a - b > 0, &ab);
    let minusplus: i64 = filter_fold(|(a, b)| -a+b > 0, &ab);
    let minusminus: i64 = filter_fold(|(a, b)| -a-b >0, &ab);

    [plusplus, plusminus, minusplus, minusminus].into_iter().max().unwrap()
}

fn filter_fold<F>(f: F, ab: &Vec<(i64, i64)>) -> i64 
where
    F: Fn(&(i64, i64)) -> bool,
{
    let fold_pair = ab.iter()
        .filter(|ab_i| f(ab_i))
        .fold((0, 0), |(acc_x, acc_y), (x, y)| (acc_x + x, acc_y + y));


    fold_pair.0.abs() + fold_pair.1.abs()
}