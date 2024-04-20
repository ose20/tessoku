use proconio::input;

fn main() {
    input! {
        d: usize,
        x: usize,
        a: [i32; d-1],
        q: usize,
        st: [(usize, usize); q],
    }

    let mut price = vec![x; d];
    for i in 1..d {
        price[i] = (price[i - 1] as i32 + a[i - 1]) as usize;
    }

    for &(s, t) in st.iter() {
        if price[s - 1] == price[t - 1] {
            println!("Same")
        } else if price[s-1] > price[t-1] {
            println!("{}", s);
        } else {
            println!("{}", t);
        }
    }
}
