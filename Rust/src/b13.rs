use std::ops::{Add, Sub};

use proconio::input;

struct CumulativeSum<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    size: usize,
    cum_sum: Vec<T>,
}

impl<T> CumulativeSum<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy
{
    fn new(v: & Vec<T>) -> CumulativeSum<T> {
        let size = v.len();
        let mut cum_sum = v.clone();
        for i in 1..size {
            cum_sum[i] = cum_sum[i] + cum_sum[i-1];
        }
        
        CumulativeSum { size, cum_sum, }
    }

    // [l, r]の区間和を求める
    fn get_interval_sum(&self, left: usize, right: usize) -> Result<T, String> {
        if left > right {
            Err(String::from("区間が不正です: left > right"))
        } else if right >= self.size {
            Err(String::from("区間が不正です: right >= size"))
        } else if left == 0 {
            Ok(self.cum_sum[right])
        } else {
            Ok(self.cum_sum[right] - self.cum_sum[left-1])
        }
    }
}



fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    solve(k, a);
}

fn solve(k: i64, a: Vec<i64>) {
    let cum_sum = CumulativeSum::new(&a);

    let mut ans: i64 = 0;
    let mut right: i64 = 0;
    for left in 0..(a.len() as i64) {
        if right < left {
            right = left;
        }

        while right < a.len() as i64 && cum_sum.get_interval_sum(left as usize, right as usize).unwrap() <= k {
            right += 1;
        }

        ans += right - left;
    }

    println!("{}", ans);
}
