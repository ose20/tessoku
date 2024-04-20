use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        st: [(Chars, usize); n]
    }

    let candidates = (0..10000)
        .filter(|&n| {
            let jackpot = to_tikect(n);
            st.iter()
                .all(|(ticket, rank)| check(&jackpot, &ticket, *rank))
        })
        .collect::<Vec<_>>();

    if candidates.len() == 1 {
        let ans = to_tikect(candidates[0]).iter().collect::<String>();
        println!("{}", ans);
    } else {
        println!("Can't Solve");
    }
}

fn to_tikect(mut n: usize) -> Vec<char> {
    let mut ticket = vec![];
    for i in 0..4 {
        ticket.push(char::from_digit((n % 10) as u32, 10).unwrap());
        n /= 10;
    }
    ticket
}

fn check(jackpot: &Vec<char>, ticket: &Vec<char>, rank: usize) -> bool {
    let mut count = 0;
    for i in 0..4 {
        if jackpot[i] != ticket[i] {
            count += 1;
        }
    }

    match count {
        0 => rank == 1,
        1 => rank == 2,
        _ => rank == 3,
    }
}
