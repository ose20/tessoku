use proconio::input;

fn main() {
    input! {
        mut x: i32,
        mut y: i32,
    }

    let ans = solve(x, y);
    println!("{}", ans.len());
    for &(x, y) in ans.iter() {
        println!("{} {}", x, y);
    }
}

fn solve(mut x: i32, mut y: i32) -> Vec<(i32, i32)> {
    let mut ans = Vec::new();

    while (x, y) != (1, 1) {
        ans.push((x, y));
    
        if x > y {
            x = x - y;
        } else {
            // x < y
            // 条件的に x == y にはならない
            y = y - x;
        }
    }

    ans.reverse();
    ans
}
