use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let flg = (0..n).any(|i| {
        (i+1..n).any(|j| {
            (j+1..n).any(|k| a[i] + a[j] + a[k] == 1000)
        })
    });

    if flg {
        println!("Yes");
    } else {
        println!("No");
    }

}
