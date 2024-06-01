use std::cmp::Reverse;

use proconio::input;

const MAX_TIME: usize = 86400 * 2 + 100;

fn main() {
    input! {
        n: usize, k: usize,
        lr: [(usize, usize); n],
    }

    let lr = lr.iter().map(|(l, r)| (*l, r + k)).collect::<Vec<_>>();

    let cnt_left = get_cnt_left(n, &lr);
    let cnt_right = get_cnt_right(n, &lr);

    for &(l, r) in lr.iter() {
        let ans = cnt_left[l] + 1 + cnt_right[r];
        println!("{}", ans);
    }
}

fn get_cnt_left(n: usize, lr: &[(usize, usize)]) -> Vec<usize> {
    // cnt_left を求める
    // cnt_left[i] := 時刻 i までの終わる会議に重複せず参加できる数の最大値
    let mut cnt_left = vec![0; MAX_TIME];

    // (l, r) を r の小さい順にソート
    let mut rl = lr.into_iter().map(|(l, r)| (*r, *l)).collect::<Vec<_>>();
    rl.sort();

    // 区間スケジューリング問題を解く
    let mut booked_time = 0;
    let mut cnt = 0;
    for &(r, l) in rl.iter() {
        if booked_time <= l {
            cnt += 1;
            cnt_left[r] = cnt;
            booked_time = r;
        }
    }

    for i in 1..MAX_TIME {
        cnt_left[i] = cnt_left[i].max(cnt_left[i - 1]);
    }

    cnt_left
}

fn get_cnt_right(n: usize, lr: &[(usize, usize)]) -> Vec<usize> {
    // cnt_right を求める
    // cnt_right[i] := 時刻 i 以降に始まる会議に重複せず参加できる数の最大値
    let mut cnt_right = vec![0; MAX_TIME];

    // (l, r) を l の大きい順にソート
    let mut lr = lr
        .into_iter()
        .map(|(l, r)| (Reverse(*l), *r))
        .collect::<Vec<_>>();
    lr.sort();

    // 区間スケジューリング問題を解く
    let mut booked_time = MAX_TIME;
    let mut cnt = 0;
    for &(Reverse(l), r) in lr.iter() {
        if r <= booked_time {
            cnt += 1;
            cnt_right[l] = cnt;
            booked_time = l;
        }
    }

    for i in (0..MAX_TIME - 1).rev() {
        cnt_right[i] = cnt_right[i].max(cnt_right[i + 1]);
    }

    cnt_right
}
