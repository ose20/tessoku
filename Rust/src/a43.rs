use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        ab: [(i32, char); n],
    }

    let calc_dist = |(a, b)| -> i32 {
        match b {
            'E' => l - a,
            'W' => a,
            _ => unreachable!()
        }
    };

    println!("{}", ab.iter().map(|t| calc_dist(*t)).max().unwrap());

}
