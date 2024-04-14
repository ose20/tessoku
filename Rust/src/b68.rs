use proconio::input;

use crate::max_flow::FordFulkerson;

// 最大流 Ford Fulkerson

pub mod max_flow {

    #[derive(Clone)]
    struct Edge {
        // 重みが capa である有向辺 from-to を表現する(隣接リストで管理する前提なので from は持たない)
        // rev: この辺の逆辺を参照するときの index
        to: usize,
        capa: usize,
        rev: usize,
    }

    impl Edge {
        fn new(to: usize, capa: usize, rev: usize) -> Edge {
            Edge { to, capa, rev }
        }
    }

    pub struct FordFulkerson {
        n: usize,
        graph: Vec<Vec<Edge>>,
    }

    impl FordFulkerson {
        // (from, to, capacity) のリストをもらって初期化する
        // 頂点番号は 0 ~ n-1 を想定している (0-indexed)
        pub fn new(n: usize) -> Self {
            FordFulkerson {
                n,
                graph: vec![vec![]; n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, capa: usize) {
            // from-toの逆辺が追加されるインデックス
            let rev_for_from = self.graph[to].len();
            // from-toが追加されるインデックス
            let rev_for_to = self.graph[from].len();
            self.graph[from].push(Edge::new(to, capa, rev_for_from));
            self.graph[to].push(Edge::new(from, 0, rev_for_to));
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
            let mut flow = 0;
            loop {
                let mut visited = vec![false; self.n];
                let f = Self::dfs(&mut self.graph, s, t, usize::MAX, &mut visited);
                if f == 0 {
                    return flow;
                }
                flow += f;
            }
        }

        fn dfs(
            graph: &mut Vec<Vec<Edge>>,
            v: usize,
            t: usize,
            flow: usize,
            visited: &mut Vec<bool>,
        ) -> usize {
            if v == t {
                return flow;
            }
            visited[v] = true;
            for i in 0..graph[v].len() {
                let edge = graph[v][i].clone();
                if !visited[edge.to] && edge.capa > 0 {
                    let d = Self::dfs(graph, edge.to, t, flow.min(edge.capa), visited);
                    if d > 0 {
                        //edge.capa -= d;
                        graph[v][i].capa -= d;
                        graph[edge.to][edge.rev].capa += d;
                        return d;
                    }
                }
            }

            0
            //　ここで visited[v] = false をしなくていい理由（計算量が増える(だけ？)ので、しなくていいならしないほうがいい）
            // 次の命題を示せば十分である
            // s: スタート地点、 t: ゴール地点、 V: 頂点集合
            // 命題: 任意のパス s-x に対して
            //          パス s-x-t がこのアルゴリズムにおいて存在しない（同じ頂点を二度通れない） & パス s-t が存在するならば
            //      x を含まないパス　s-t が存在する
            // 証明:
            //  パス s-x を任意に取り（パス1と名付ける）、パス s-x-t が存在せず、パス s-t が存在することを仮定する
            //  この時、パス s-x の中継地点になってる頂点集合 U が取れる
            //  パス s-t が存在するとして、そのうち x が含まれるものは、x が含まれないパスに再構成できることを示す
            //  x が含まれるとして、経路 x-t の x と t の間にある頂点 y に注目する
            //  - x-t の間の頂点に少なくとも　1　つ U の頂点がある場合
            //      t 側から見ていって初めて　U の要素となる頂点を u とする（パス u-tが作れ、その中継点には U の要素も x も含まれない）。
            //      u はその定義よりパス1である s-x の中継点なので、s-u-t が作れる（これは x を含まない）
            //  - x-t の間に U の頂点が 1 つもない場合
            //      このパスとパス 1 を繋げることで、s-x-t が作れてしまって仮定に矛盾するのでこのケースは起きえない
        }
    }
}

// 公式の解説の行間が大きすぎるので少し補足する
// s-tカットを作るなら、任意の i(1<=i<=n) に対して、辺 s-i か辺 i-t のいずれか一つのみがカット容量に含まれるはず
// これはsとtは分断されていないといけないからで、このことが、i に対して特急駅を設けるか否かという選択に対応している
// というか、これは自然にs-tカットを取ろうとしたら、残りの頂点1~nはsかtかどちらと同じグループにするかしかないので2^n通りだよねという話
// つまりs-tカットの選び方は 2^n 個ある
// このうち、条件を満たさないような選び方に対応するカットの容量は必ず無限になることを示せればいいだろう
// 解説の例で考えると、1に作るのに2に作らない場合、1はsと同じグループに入り、　２はtと同じグループになる
// すると1-2の重さ ∞ はこのカット容量に含まれるので、最小カットにはなり得ない
fn main() {
    input! {
        n: usize, m: usize,
        p: [i64; n],
        ab: [(usize, usize); m],
    }

    let inf = usize::MAX / 20;
    let mut ff = FordFulkerson::new(n + 2);
    let mut offset = 0;
    for (i, &pi) in p.iter().enumerate() {
        if pi >= 0 {
            offset += pi;
            ff.add_edge(0, i + 1, pi as usize)
        } else {
            ff.add_edge(i + 1, n + 1, -pi as usize)
        }
    }

    for &(a, b) in ab.iter() {
        ff.add_edge(a, b, inf);
    }

    let min_cost = ff.max_flow(0, n + 1);
    println!("{}", offset as usize - min_cost);
}
