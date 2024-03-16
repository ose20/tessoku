use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, i32); n],
    }
    let d = 10000;

    let proc = |acc: i32, &(op, num) : &(_, i32)| {
        match op {
            '+' => {
                let tmp = (acc + num) % d;
                println!("{}", tmp);
                tmp
            }
            '-' => {
                let tmp = (acc - num + d) % d;
                println!("{}", tmp);
                tmp
            }
            '*' => {
                let tmp = (acc * num) % d;
                println!("{}", tmp);
                tmp
            }
            _ => unreachable!()
        }
    };

    let _ans = ta.iter().fold(0, proc);

}
