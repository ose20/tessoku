use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let primes = collect_primes(n);
    primes.iter().for_each(|p| println!("{}", p));
}

fn collect_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut table = vec![true; n+1];
    table[1] = false;

    for i in 2..=n {
        if table[i] { 
            primes.push(i); 
            // iの倍数でtableをチェックする
            {
                let mut cur = 2*i;
                loop {
                    if cur > n { break; }
                    else {
                        table[cur] = false;
                        cur += i;
                    }
                }
            }            
        }
    }

    primes 
}

