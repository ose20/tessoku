use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // n-1 を2進数にして、0->4, 1->7に置換すればいい
    println!(
        "{}",
        to_bin_list(n - 1)
            .iter()
            .map(|&b| if b { "7" } else { "4" })
            .collect::<Vec<_>>()
            .join("")
    )
}

fn to_bin_list(n: usize) -> Vec<bool> {
    let mut ans = vec![false; 10];
    for i in 0..10 {
        if (1 << i) & n != 0 {
            ans[9 - i] = true
        } else {
            ans[9 - i] = false
        }
    }

    ans
}
