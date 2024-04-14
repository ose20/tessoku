use proconio::input;

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

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, usize); m]
    }

    let mut ans = 0;
    let mut uf = UnionFind::init(n);
    abc.sort_by_key(|&(_, _, third)| third);

    for &(a, b, c) in abc.iter() {
        if !uf.is_same(a - 1, b - 1) {
            ans += c;
            uf.unite(a - 1, b - 1);
        }
    }

    println!("{}", ans);
}
