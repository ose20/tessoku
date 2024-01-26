use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    }

    if solve(k, a, b, c, d) {
        println!("Yes");
    } else {
        println!("No");
    }

}

fn solve(k: i64, a: Vec<i64>, b: Vec<i64>, c: Vec<i64>, d: Vec<i64>) -> bool {

    let mut ab = a.iter()
        .flat_map(|ai| b.iter().map(move |bi| ai + bi));


    let mut cd : Vec<i64> = c.iter()
        .flat_map(|ci| d.iter().map(move |di| ci + di))
        .collect();

    cd.sort();

    ab.any(|x| cd.binary_search(&(k - x)).is_ok())        

}
