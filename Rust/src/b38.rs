use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    solve(n, s);
}

fn solve(n: usize, s: Vec<char>) {
    let mut ret1 = vec![0; n];
    ret1[0] = 1;
    let mut streak1 = 1;
    for i in 0..n-1 {
        if s[i] == 'A' { streak1 += 1; }
        if s[i] == 'B' { streak1 = 1; }
        ret1[i + 1] = streak1;
    }

    let mut ret2 = vec![0; n];
    ret2[n-1] = 1;
    let mut streak2 = 1;
    for i in (0..n-1).rev() {
        if s[i] == 'B' { streak2 += 1; }
        if s[i] == 'A' { streak2 = 1; }
        ret2[i] = streak2;
    }

    let ans: i32 = (0..n).map(|i| ret1[i].max(ret2[i])).sum();
    println!("{}", ans);

}

