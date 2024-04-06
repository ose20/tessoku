use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }

    solve(n, &xy);
}

fn solve(n: usize, xy: &[(f64, f64)]) {
    let dist = calc_dist(n, xy);
    let path = construct_path(n, &dist);

    path.iter().for_each(|idx| println!("{}", idx + 1));
}

fn calc_dist(n: usize, xy: &[(f64, f64)]) -> Vec<Vec<f64>> {
    let mut dist = vec![vec![0.0; n]; n];
    for i in 0..n {
        let (xi, yi) = xy[i];
        for j in 0..n {
            let (xj, yj) = xy[j];
            let square = (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj);
            dist[i][j] = square.sqrt();
        }
    }

    dist
}

fn construct_path(n: usize, dist: &Vec<Vec<f64>>) -> Vec<usize> {
    let mut path = vec![0; n + 1];
    let mut visited = vec![false; n + 1];
    visited[0] = true;

    let mut cur = 0;
    for i in 1..=n - 1 {
        // n-1回のループで path が埋まる
        // 未訪問の頂点から cur との距離が最小のものを探す
        let (_, next) = (0..n)
            .filter(|idx| !visited[*idx])
            .map(|idx| (dist[cur][idx], idx))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        path[i] = next;
        cur = next;
        visited[next] = true;
    }

    path
}
