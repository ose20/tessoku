use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }

    solve(a);
}

fn solve(a: Vec<i32>) {
    let comp = compress(a);
    
    let ans: String = comp.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", ans);
        
}

fn compress(a: Vec<i32>) -> Vec<usize> {
    let mut b = a.clone();

    b.sort();
    b.dedup();

    a.iter().map(|ai| b.binary_search(ai).unwrap() + 1).collect()
}
