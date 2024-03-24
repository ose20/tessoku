use proconio::input;

fn main() {
    input! {
        n: usize,
        _x: usize,
        _y: usize,
        a: [usize; n],
    }

    let xor_sum = a.iter()
        .map(|ai| {
            if ai%5 == 0 || ai%5 == 1 { 0 }
            else if ai%5 == 2 || ai%5 == 3 { 1 }
            else { 2 }
        })
        .fold(0, |acc, grundy| acc ^ grundy);

    if xor_sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
