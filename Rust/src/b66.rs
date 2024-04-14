use std::{collections::HashMap, hash::Hash};

use proconio::input;

// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// ------------------------------------------------------------
// UnifonFind
struct UnionFind {
    // 親の頂点番号
    par: Vec<Option<usize>>,
    // 要素が属する根つき木の高さ
    // 経路圧縮しているので実態は違うけど、経路圧縮しなかった場合の値を持っておく
    rank: Vec<usize>,
    // 要素と同じ集合に含まれる頂点の数
    size: Vec<usize>,
}

impl UnionFind {
    fn init(n: usize) -> Self {
        Self {
            par: vec![None; n],
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x].is_none() {
            x
        } else {
            self.par[x] = Some(self.root(self.par[x].unwrap()));
            self.par[x].unwrap()
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        let (rx, ry) = {
            let rx = self.root(x);
            let ry = self.root(y);
            if self.rank[rx] < self.rank[ry] {
                (ry, rx)
            } else {
                (rx, ry)
            }
        };

        if rx == ry {
            return;
        }

        self.par[ry] = Some(rx);
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }

        self.size[rx] += self.size[ry]
    }

    fn size(&mut self, x: usize) -> usize {
        let rt = self.root(x);
        self.size[rt]
    }
}

#[derive(Clone, Copy)]
enum Query {
    Break(usize),
    IsConnected(usize, usize),
}

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
        q: usize
    }

    let queries = parse_query(q);
    let broken_id = collect_broken(m, &queries);
    solve(n, m, &queries, &ab, &broken_id);
}

fn parse_query(q: usize) -> Vec<Query> {
    (0..q)
        .map(|_| {
            input! { n: usize }
            match n {
                1 => {
                    input! { x: usize }
                    Query::Break(x - 1)
                }
                2 => {
                    input! { x: usize, y: usize }
                    Query::IsConnected(x - 1, y - 1)
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

fn collect_broken(m: usize, queries: &[Query]) -> Vec<bool> {
    let mut broken = vec![false; m];
    queries.iter().for_each(|q| match q {
        &Query::Break(i) => {
            broken[i] = true;
        }
        _ => {}
    });
    broken
}

fn solve(n: usize, _m: usize, queries: &[Query], ab: &[(usize, usize)], is_broken: &[bool]) {
    let mut uf = UnionFind::init(n);

    // クエリで壊れないレールを全て繋ぐ
    for (i, &(a, b)) in ab.iter().enumerate() {
        if !is_broken[i] {
            uf.unite(a - 1, b - 1);
        }
    }

    let mut ans = vec![];

    for query in queries.iter().rev() {
        match query {
            &Query::Break(i) => uf.unite(ab[i].0 - 1, ab[i].1 - 1),
            &Query::IsConnected(x, y) => {
                if uf.is_same(x, y) {
                    ans.push("Yes");
                } else {
                    ans.push("No");
                }
            }
        }
    }

    ans.reverse();
    ans.iter().for_each(|s| println!("{}", s));
}
