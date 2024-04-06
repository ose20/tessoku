use core::fmt;

use proconio::input;

#[derive(Clone, Copy, Debug)]
enum Move {
    A,
    B,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Move::A => write!(f, "A"),
            Move::B => write!(f, "B"),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    // これまでの合計スコア
    score: usize,
    // 今の状態
    x: Vec<i32>,
    // どのムーブをしてこの状態になっているか
    last_move: Option<Move>,
    // 直前の状態があるidx
    last_idx: usize,
}

fn main() {
    input! {
        t: usize,
        pqr: [(usize, usize, usize); t],
    }

    solve(t, &pqr);
}

fn solve(t: usize, pqr: &[(usize, usize, usize)]) {
    let beam_size = 100000;

    let mut states = Vec::new();
    let initial_state = State {
        score: 0,
        x: vec![0; 20],
        last_move: None,
        last_idx: 0,
    };
    states.push(vec![initial_state; 1]);

    for i in 0..t {
        let (pi, qi, ri) = pqr[i];
        // states の最後の状態列
        let mut this_states = Vec::new();
        for (j, last_state) in states.last().unwrap().iter().enumerate() {
            // last_state から move A を選択した時の State を得る
            this_states.push(get_next_state(last_state, &Move::A, (pi, qi, ri), j));
            // last_state から move B を選択した時の State を得る
            this_states.push(get_next_state(last_state, &Move::B, (pi, qi, ri), j));
        }

        this_states.sort_by(|a, b| a.score.cmp(&b.score));
        this_states.reverse();
        states.push(this_states.into_iter().take(beam_size).collect());
    }

    let path = construct_path(&states);

    path.iter().for_each(|m| println!("{}", m));
}

fn get_next_state(state: &State, m: &Move, (p, q, r): (usize, usize, usize), j: usize) -> State {
    let mut nxt_x = state.x.clone();
    match m {
        Move::A => {
            nxt_x[p] += 1;
            nxt_x[q] += 1;
            nxt_x[r] += 1;
        }
        Move::B => {
            nxt_x[p] -= 1;
            nxt_x[q] -= 1;
            nxt_x[r] -= 1;
        }
    }

    let nxt_score = state.score + nxt_x.iter().filter(|v| **v == 0).count();

    State {
        score: nxt_score,
        x: nxt_x,
        last_move: Some(*m),
        last_idx: j,
    }
}

fn construct_path(states: &Vec<Vec<State>>) -> Vec<Move> {
    let mut path = Vec::new();

    let mut last_idx = 0;
    for i in (1..states.len()).rev() {
        let (m, j) = (states[i][last_idx].last_move, states[i][last_idx].last_idx);
        path.push(m.unwrap());
        last_idx = j;
    }

    path.reverse();
    path
}
