use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut ans: Vec<i32> = Vec::new();
    for (i, ai) in a.iter().enumerate() {
        // stack の中を遡り、ai 以下のものが続く限り、それを削除し続ける
        while !stack.is_empty() {
            let price = stack.last().unwrap().1;
            if price <= *ai {
                stack.pop();
            } else {
                break;
            }
        }

        // 決算日の特定
        if !stack.is_empty() {
            ans.push(stack.last().unwrap().0 as i32)
        } else {
            ans.push(-1);
        }

        stack.push((i + 1, *ai));
    }

    println!(
        "{}",
        ans.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
