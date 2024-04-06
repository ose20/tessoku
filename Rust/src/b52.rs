use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: usize,
        a: Chars,
    }

    let ans = solve(n, x, a);
    println!("{}", ans.iter().collect::<String>());
}

fn solve(n: usize, x: usize, mut a: Vec<char>) -> Vec<char> {
    a[x - 1] = '@';
    let mut left: i32 = x as i32 - 1 - 1;
    let mut right: i32 = x as i32 - 1 + 1;
    while left >= 0 && a[left as usize] == '.' {
        a[left as usize] = '@';
        left -= 1;
    }

    while right < n as i32 && a[right as usize] == '.' {
        a[right as usize] = '@';
        right += 1;
    }

    a
}
