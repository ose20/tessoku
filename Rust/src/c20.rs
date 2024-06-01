use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, k: usize, l: usize,
        ab: [(usize, usize); k],
        c: [[usize; n]; n],
    }

    let adj = create_adj(n, k, &c);

    // 都市 1 と連結している k-l 個（都市 1 を入れて k-l+1）の年を任意に選び、特別区 1 に合併する
    // 残りの l-1 個の都市は、それぞれ特別区 2 ~ l に割り当てる

    // adj グラフを与えて、都市 1 と連結している k-l+1 個の都市のリストを得る
    let region1: HashSet<usize> = get_region1(&adj, k - l).into_iter().collect();

    //let mut counter = 2;
    //for i in 1..=k {
    //    if region1.contains(&i) {
    //        println!("1");
    //    } else {
    //        println!("{}", counter);
    //        counter += 1;
    //    }
    //}
}

fn create_adj(n: usize, k: usize, c: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // adj[i] := 都市 i と連結している都市のリスト
    // 都市は 0 ~ k まである（ 0 は特別な都市で、存在しない）
    let mut adj = vec![vec![]; k + 1];

    for i in 0..n {
        for j in 0..n {
            // ↓ 方向の連結をチェック
            if i < n - 1 && c[i][j] != 0 && c[i + 1][j] != 0 && c[i][j] != c[i + 1][j] {
                adj[c[i][j]].push(c[i + 1][j]);
                adj[c[i + 1][j]].push(c[i][j]);
            }

            // → 方向の連結をチェック
            if j < n - 1 && c[i][j] != 0 && c[i][j + 1] != 0 && c[i][j] != c[i][j + 1] {
                adj[c[i][j]].push(c[i][j + 1]);
                adj[c[i][j + 1]].push(c[i][j]);
            }
        }
    }

    for i in 1..=k {
        adj[i].sort();
        adj[i].dedup();
    }
    adj
}

// 都市 1 を含む部分グラフを都市 n 個（都市 1 以外で）取り出して、その都市のリスト（1を含む）を返す
fn get_region1(adj: &Vec<Vec<usize>>, n: usize) -> Vec<usize> {
    let mut res = vec![];

    let size = adj.len();
    let mut seen = vec![false; size];

    dfs(adj, 1, 0, n + 1, &mut seen, &mut res);

    println!("{:?}", res.len());
    res
}

fn dfs(
    adj: &Vec<Vec<usize>>,
    now_v: usize,
    cnt: usize,
    limit: usize,
    seen: &mut [bool],
    res: &mut Vec<usize>,
) {
    res.push(now_v);
    seen[now_v] = true;

    for next in adj[now_v].iter() {
        if !seen[*next] && res.len() < limit {
            dfs(adj, *next, cnt, limit, seen, res)
        }
    }
}
