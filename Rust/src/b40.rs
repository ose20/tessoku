use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let ans = solve(&a);
    println!("{}", ans);   
}

fn solve(a: &[usize]) -> i32 {
    let mut map: HashMap<usize, i32> = HashMap::new();
    
    for ai in a.iter() {
        *map.entry(ai % 100).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..=50 {
        let get_i = map.get(&i).unwrap_or(&0);
        if i == 0 || i == 50 {
            ans += get_i * (get_i - 1) / 2;
        } else {
            ans += get_i * (map.get(&(100 - i)).unwrap_or(&0))
        }
    }

    ans
}
