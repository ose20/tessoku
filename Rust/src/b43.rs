use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let ans = solve(n, m, &a);
    for s in ans {
        println!("{}", s);
    }
}

fn solve(n: usize, m: usize, a: &[usize]) -> Vec<usize> {
    let mut res = vec![m; n];
    
    for ai in a.iter() {
        res[ai - 1] -= 1;
    }
    
    res
}
