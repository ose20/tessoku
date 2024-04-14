use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        xyz: [(usize, usize, usize); m],
    }

    let start = to_state_num(&a);
    let adj = create_adj(n, &xyz);
    let dist = calc_dist(start, &adj);
    let ans = {
        let tmp = dist[(1 << n) - 1];
        if tmp == usize::MAX {
            -1
        } else {
            tmp as i64
        }
    };
    println!("{}", ans);
}

// {0,1} からなる [a0, a1, ..., a(n-1)] を 2^0 * a0 + ... + 2^(n-1) * a(n-1) に変換する
fn to_state_num(vec: &[usize]) -> usize {
    let mut num = 0;
    for i in 0..vec.len() {
        if vec[i] == 1 {
            num += 1 << i;
        }
    }

    num
}

fn create_adj(n: usize, xyz: &[(usize, usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; 1 << n];

    for state in 0..1 << n {
        for &(x, y, z) in xyz.iter() {
            // state から x,y,z 番目のランプを反転させた状態に辺が生える
            let nxt_state = state ^ (1 << (x - 1)) ^ (1 << (y - 1)) ^ (1 << (z - 1));
            adj[state].push(nxt_state);
        }
    }

    adj
}

fn calc_dist(start: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let inf = usize::MAX;
    let mut dist = vec![inf; adj.len()];

    let mut que = VecDeque::new();
    que.push_back(start);
    dist[start] = 0;
    while !que.is_empty() {
        let now = que.pop_front().unwrap();
        for &next in adj[now].iter() {
            if dist[next] == inf {
                dist[next] = dist[now] + 1;
                que.push_back(next);
            }
        }
    }

    dist
}
