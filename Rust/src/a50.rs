use std::time::Instant;

use proconio::input;
use rand::Rng;

const N: usize = 100;
const Q: usize = 1000;

type Board = Vec<Vec<i64>>;

struct Solution {
    x: Vec<i32>,
    y: Vec<i32>,
    h: Vec<i32>,
}

fn main() {
    input! {
        a: [[i64; N]; N],
    }

    let timer = Instant::now();
    solve(&a, &timer);
}

fn solve(a: &Vec<Vec<i64>>, timer: &Instant) {
    let (mut solution, mut board) = init_solution();

    let limit: f64 = 5.9;
    let mut current_score = 0;
    while timer.elapsed().as_secs_f64() < limit {
        let kth = random_with_range(0, (Q - 1) as i32);
        let (old_x, new_x) = {
            let x = solution.x[kth as usize];
            (x, x + random_with_range(-9, 9))
        };
        let (old_y, new_y) = {
            let y = solution.y[kth as usize];
            (y, y + random_with_range(-9, 9))
        };
        let (old_h, new_h) = {
            let h = solution.h[kth as usize];
            (h, h + random_with_range(-19, 19))
        };
        if !(0..N as i32).contains(&new_x)
            || !(0..N as i32).contains(&new_y)
            || !(1..N as i32).contains(&new_h)
        {
            continue;
        }

        update_solution(kth as usize, new_x, new_y, new_h, &mut solution, &mut board);
        let new_score = calc_score(&board, &a);
        if current_score < new_score {
            current_score = new_score;
        } else {
            update_solution(kth as usize, old_x, old_y, old_h, &mut solution, &mut board);
        }
    }

    println!("{}", Q);
    for i in 0..Q {
        println!("{} {} {}", solution.x[i], solution.y[i], solution.h[i]);
    }
}

fn calc_score(a: &Board, b: &Board) -> i64 {
    let sum = (0..N)
        .flat_map(|i| (0..N).map(move |j| (a[i][j] - b[i][j]).abs()))
        .sum::<i64>();

    2_0000_0000 - sum
}

fn init_solution() -> (Solution, Board) {
    let mut board = vec![vec![0; N]; N];
    let mut x = vec![0; Q];
    let mut y = vec![0; Q];
    let mut h = vec![0; Q];

    for i in 0..Q {
        x[i] = random_with_range(0, (N as i32) - 1);
        y[i] = random_with_range(0, (N as i32) - 1);
        h[i] = 1;
        board[y[i] as usize][x[i] as usize] += 1;
    }

    (Solution { x, y, h }, board)
}

fn random_with_range(left: i32, right: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(left..=right)
}

fn update_solution(
    kth: usize,
    x_val: i32,
    y_val: i32,
    h_val: i32,
    solution: &mut Solution,
    board: &mut Board,
) {
    // kth 番目の山の、Board への寄与分を引く
    for i in 0..N {
        for j in 0..N {
            board[i][j] -= 0.max(
                solution.h[kth]
                    - (solution.x[kth] - j as i32).abs()
                    - (solution.y[kth] - i as i32).abs(),
            ) as i64
        }
    }

    solution.x[kth] = x_val;
    solution.y[kth] = y_val;
    solution.h[kth] = h_val;

    // kth 番目の山の、　Borad への寄与分を足す
    for i in 0..N {
        for j in 0..N {
            board[i][j] += 0.max(
                solution.h[kth]
                    - (solution.x[kth] - j as i32).abs()
                    - (solution.y[kth] - i as i32).abs(),
            ) as i64
        }
    }
}
