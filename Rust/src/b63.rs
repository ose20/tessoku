use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize, c: usize,
        sy: usize, sx: usize,
        gy: usize, gx: usize,
        maze: [Chars; r],
    }

    let dist = bfs((sy - 1, sx - 1), &maze, r, c);
    println!("{}", dist[encode(gy - 1, gx - 1, c)]);

    //for i in 0..r {
    //    for j in 0..c {
    //        let d = dist[encode(i, j, c)];
    //        print!("{:>5}", if d == usize::MAX { -1 as i64 } else { d as i64 })
    //    }
    //    println!("");
    //}
}

fn encode(h: usize, w: usize, base: usize) -> usize {
    h * base + w
}

fn next(h: usize, w: usize) -> impl Iterator<Item = (usize, usize)> {
    let d1 = [-1, 0, 1, 0];
    let d2 = [0, 1, 0, -1];

    (0..4)
        .into_iter()
        .map(move |i| ((h as i32 + d1[i]) as usize, (w as i32 + d2[i]) as usize))
}

fn bfs((sh, sw): (usize, usize), maze: &Vec<Vec<char>>, h: usize, w: usize) -> Vec<usize> {
    let inf = usize::MAX;
    let mut dist = vec![inf; h * w];
    let base = w;

    let mut que = VecDeque::new();
    dist[encode(sh, sw, base)] = 0;
    que.push_back((sh, sw));

    while !que.is_empty() {
        let (now_h, now_w) = que.pop_front().unwrap();
        for (next_h, next_w) in next(now_h, now_w) {
            let next_idx = encode(next_h, next_w, base);
            if maze[next_h][next_w] == '.' && dist[next_idx] == inf {
                dist[next_idx] = dist[encode(now_h, now_w, base)] + 1;
                que.push_back((next_h, next_w));
            }
        }
    }

    dist
}
