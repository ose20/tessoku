use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        ab: [(i32, i32); n],
    }

    println!("{}", solve(n, k, &ab));
}

fn solve(_n: usize, k: i32, ab: &Vec<(i32, i32)>) -> i32 {
    let mut ans = 0;

    for health in 1..=100 {
        for guts in 1..=100 {
            let tmp = ab.iter().filter(|(a, b)| {
                health <= *a && *a <= health + k && guts <= *b && *b <= guts + k
            }).count();
            ans = ans.max(tmp);
        }
    }

    ans as i32
}
