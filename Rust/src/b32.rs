use proconio::input;
use Player::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    A,
    B,
}

impl Player {
    fn other(&self) -> Self {
        match self {
            A => B,
            B => A,
        }
    }
}

impl From<usize> for Player {
    fn from(n: usize) -> Self {
        match n {
            0 => A,
            1 => B,
            _ => unreachable!()
        }
    }
}


impl From<Player> for usize {
    fn from(player: Player) -> Self {
        match player {
            A => 0,
            B => 1,
        }
    }
}


fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let dp = calc_dp(n, k, &a);
    match dp[n][usize::from(A)] {
        A => println!("First"),
        B => println!("Second"),
    }
}


fn calc_dp(n: usize, _k: usize, a: & Vec<usize>) -> Vec<Vec<Player>> {
    // dp[i][j] := 残り　 i 個の石が積まれている状態でターンが Player::from(j) の時、どちらが勝つか
    let mut dp = vec![vec![A; 2]; n+1];

    for i in 0..=n {
        for j in 0..=1 {
            let turn_p = Player::from(j);
            let other_p = turn_p.other();

            // 石 ai 個とる選択肢の中から勝ち筋があれば turn_p の勝ち、なければ other_p の勝ち
            let win = a.iter().any(|&ai| {
                ai <= i && dp[i - ai][usize::from(other_p)] == turn_p
            });

            if win { dp[i][j] = turn_p; }
            else { dp[i][j] = other_p; }

        }
    }

    dp
}