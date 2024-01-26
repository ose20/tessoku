use proconio::input;

fn main() {
    input! {
        d: u32,
        n: u32,
        lr: [(u32, u32); n]
    }

    let imos = calc_imos(d, lr);

    for i in 1..=(d as usize) {
        println!("{}", imos[i]);
    }
    
}

fn calc_imos(d: u32, lr: Vec<(u32, u32)>) -> Vec<i32> {
    let mut imos = vec![0; (d as usize) + 2];

    for &(l, r) in lr.iter() {
        imos[l as usize] += 1;
        imos[(r+1) as usize] -= 1;
    }

    for i in 1..=(d as usize) {
        imos[i] += imos[i-1];
    }

    imos   
}
