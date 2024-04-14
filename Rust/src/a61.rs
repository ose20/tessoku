use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n + 1];
    for &(a, b) in ab.iter() {
        adj[a].push(b);
        adj[b].push(a);
    }

    for i in 1..=n {
        print!("{i}: ");
        print!("{{");
        print!(
            "{}",
            adj[i]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!("}}")
    }
}
