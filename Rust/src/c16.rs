use proconio::input;

// memo
// 最終的にほしいグラフは Vec<Vec<(usize, usizse)>> (to, cost)
// 時間の早い順にノードを番号を振りたい（ノード番号の大きい方から小さい方に辺が貼られているという前提を作れると、dpの遷移がDAG上になってやりやすいため
// ノードの並び替えに必要な順序はこれ
// 対応する時刻が早い方が早い
// 対応する時刻も空港も同じなら、arrival < departure になってる（到着してすぐに出発という動きを考慮しないといけないから）
// - これは時刻が同じなら arrival < departure としても十分である

// グラフを作るのに必要な他の情報は
// 各空港ごとに、対応するノードを時間の早い順に並べた配列（例えばこれは、フライト以外の遷移（空港で次のフライトまで待つとか）を表現する辺を張るのに十分な情報になる）

// 最初にノードを若い順に並べるときに、ノードが保持していてほしい情報(直接持っていなくても、一意に導出できればいい)（必要十分じゃなくて十分であればいい）
// 対応する時刻、対応する空港、フライトに関するノードならばそれが arrival か departure か
// フライトのノードならフライトの index があれば空港は一意に定まる

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeKind {
    Arrival(usize),   // フライトのid (0-index)
    Departure(usize), // フライトのid (0-index)
    Airport(usize),   // 空港のid (1-index)
    End,              // 大元の端の点
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        asbt: [(usize, usize, usize, usize); m],
    }

    let asbt = asbt
        .clone()
        .into_iter()
        .map(|(a, s, b, t)| (a, s, b, t + k))
        .collect::<Vec<_>>();

    let ordered_nodes = make_nodes(n, &asbt);
    let nodes_by_airport = make_nodes_by_airport(n, &asbt, &ordered_nodes);
    let map_from_flight_to_node = make_map_from_flight_to_nodeid(&asbt, &ordered_nodes);
    let adj = make_graph(
        n,
        &asbt,
        &ordered_nodes,
        &nodes_by_airport,
        &map_from_flight_to_node,
    );
    let dp = calc_dp(ordered_nodes.len(), &adj);
    println!("{}", dp.last().unwrap());
}

// 最終的に作りたいグラフに存在する全てのノードを上述の順序で整列したリストを作る
fn make_nodes(n: usize, asbt: &[(usize, usize, usize, usize)]) -> Vec<(i64, NodeKind)> {
    let mut res: Vec<(i64, NodeKind)> = vec![];
    let inf = 10_000_000_007;

    // 各空港の始点、終点ノードを追加
    for i in 1..=n {
        // 始点に対応する時刻は 0
        res.push((0, NodeKind::Airport(i)));
        // 終点に対応する時刻は　　inf
        res.push((inf, NodeKind::Airport(i)));
    }

    // フライトノードの追加
    for (i, &(_, s, _, t)) in asbt.iter().enumerate() {
        res.push((s as i64, NodeKind::Departure(i)));
        res.push((t as i64, NodeKind::Arrival(i)));
    }

    // 大元の始点と終点を入れる
    res.push((-1, NodeKind::End));
    res.push((inf * 2, NodeKind::End));

    res.sort();
    res
}

// 各空港ごとに、時刻が早い順ソートされた状態でノード番号のリストを作る(0-index)
fn make_nodes_by_airport(
    n: usize,
    asbt: &[(usize, usize, usize, usize)],
    ordered_nodes: &[(i64, NodeKind)],
) -> Vec<Vec<usize>> {
    let mut nodes_by_airport = vec![vec![]; n];

    // ordered_nodesがすでに整列されているのでこいつを最初から見て各空港に振り分ければいい
    for (node_index, &(_time, node)) in ordered_nodes.iter().enumerate() {
        match node {
            NodeKind::Arrival(i) => {
                // i番目のフライト (0-indexに直す)
                nodes_by_airport[asbt[i].2 - 1].push(node_index)
            }
            NodeKind::Departure(i) => {
                // i番目のフライト (0-indexに直す)
                nodes_by_airport[asbt[i].0 - 1].push(node_index)
            }
            NodeKind::Airport(i) => {
                // i番目の空港（始点または終点）
                nodes_by_airport[i - 1].push(node_index)
            }
            NodeKind::End => {
                // 空港のノードじゃないので何もしない
            }
        }
    }

    nodes_by_airport
}

// 以下の map を作る
// map[i] = (id1, id2) <=> i番目のフライトの出発ノードのノード番号が id1 で、到着ノードのノード番号が id2 である
fn make_map_from_flight_to_nodeid(
    asbt: &[(usize, usize, usize, usize)],
    ordered_nodes: &[(i64, NodeKind)],
) -> Vec<(usize, usize)> {
    let mut map = vec![(0, 0); asbt.len()];
    for (node_id, &(_time, nodekind)) in ordered_nodes.iter().enumerate() {
        //
        match nodekind {
            NodeKind::Departure(i) => map[i].0 = node_id,
            NodeKind::Arrival(i) => map[i].1 = node_id,
            _ => {}
        }
    }

    map
}

// 隣接リストを作る
// ただし辺は便宜上逆向きに張る
fn make_graph(
    n: usize,
    asbt: &[(usize, usize, usize, usize)],
    ordered_nodes: &[(i64, NodeKind)],
    nodes_by_airports: &Vec<Vec<usize>>,
    map_from_flight_to_node: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    // 方針:
    //  辺の種類は3つあるので、それぞれ次のように貼る
    //      1. フライトを表現する辺（異なる空港への辺）
    //          前計算しておいて map_from_flight_to_node を使う
    //      2. 同じ空港同士の辺
    //          nodes_by_airport の隣り合う辺を後ろから前に貼る
    //      3. それ以外の辺
    //          つまり大元の始点から各空港への始点への辺と、各空港の終点から大元の終点への辺
    //          これはそれぞれがノード番号の先頭と末尾に並んでいることを利用してノード番号を特定する

    let size = ordered_nodes.len();
    let mut adj = vec![vec![]; size];

    // フライトを表現する辺
    for i in 0..asbt.len() {
        // i番目のフライトの Arrival, Departure ノードのノード番号を O(1) で求められるデータ構造がほしいので map_from_flight_to_node の出番
        let (departure_id, arrival_id) = map_from_flight_to_node[i];
        adj[arrival_id].push((departure_id, 1));
    }

    // 同じ空港同士の辺
    for i in 0..n {
        for j in 0..nodes_by_airports[i].len() - 1 {
            let from = nodes_by_airports[i][j + 1];
            let to = nodes_by_airports[i][j];
            adj[from].push((to, 0));
        }
    }

    // それ以外の辺
    // 始点の　id　は　0, 各空港の始点の id は 1~n
    for i in 1..=n {
        adj[i].push((0, 0));
    }
    // 終点の id は size-1、各空港の終点の id は size-1-n ~ size-1-1
    for i in (size - 1 - n)..=(size - 1 - 1) {
        adj[size - 1].push((i, 0));
    }

    adj
}

fn calc_dp(size: usize, adj: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dp = vec![0; size];
    dp[0] = 0;

    for i in 1..size {
        for &(to, cost) in adj[i].iter() {
            dp[i] = dp[i].max(dp[to] + cost);
        }
    }

    dp
}
