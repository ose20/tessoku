use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut ans = vec![];

    for i in 1.. {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            ans.push(i);
            if n / i != i {
                ans.push(n / i);
            }
        }
    }

    ans.sort();
    ans.iter().for_each(|v| println!("{}", v));
}
