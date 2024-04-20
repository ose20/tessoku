use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
        x: [usize; q],
    }

    c.sort();
    let cum = calc_cum(n, &c);
    //println!("{:?}", cum);
    for &xi in x.iter() {
        let ans = cum.partition_point(|&sum| sum <= xi) - 1;
        println!("{}", ans);
    }
}

fn calc_cum(n: usize, c: &[usize]) -> Vec<usize> {
    let mut cum = vec![0; n + 1];

    for i in 1..=n {
        cum[i] = cum[i - 1] + c[i - 1];
    }

    cum
}
