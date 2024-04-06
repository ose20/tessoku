use proconio::input;
use rand::{thread_rng, Rng};

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
    path.iter().for_each(|idx| println!("{}", idx + 1))
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
    // 初期解
    let mut path = vec![0; n + 1];
    for i in 1..n {
        path[i] = i
    }

    let mut score = calc_score(&path, dist);

    let tmax = 500000;
    for t in 0..=tmax {
        let (left, right) = get_rand_range(n);

        path[left..=right].reverse();
        let tmp_score = calc_score(&path, &dist);

        let temp = 30.0 - 28.0 * (t as f64) / (tmax as f64);
        let diff = (score - tmp_score) as f64;
        let probability = if diff >= 0.0 {
            1.0
        } else {
            (diff / temp).exp()
        };

        if get_rand_p() < probability {
            score = tmp_score;
        } else {
            path[left..=right].reverse();
        }
    }

    path
}

// [1, n-1] から範囲をランダムに選択する
fn get_rand_range(n: usize) -> (usize, usize) {
    let mut rng = thread_rng();

    let (left, right) = (rng.gen_range(1..n), rng.gen_range(1..n));
    (left.min(right), left.max(right))
}

fn calc_score(path: &[usize], dist: &Vec<Vec<f64>>) -> f64 {
    let mut score = 0.0;
    for i in 0..path.len() - 1 {
        score += dist[path[i]][path[i + 1]];
    }
    score
}

fn get_rand_p() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
