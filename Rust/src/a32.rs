use proconio::input;
use Player::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    A,
    B,
}

impl Player {
    fn to_bin(&self) -> usize {
        match self {
            Player::A => 0,
            Player::B => 1,
        }
    }

    fn from_bin(n: usize) -> Player {
        match n {
            0 => Player::A,
            1 => Player::B,
            _ => unreachable!()
        }
    }

    fn other(&self) -> Self {
        match self {
            A => B,
            B => A,
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: i32,
        b: i32,
    }

    let dp = calc_dp(n, a, b);

    match dp[n][A.to_bin()] {
        A => println!("First"),
        B => println!("Second"),
    }

    //println!("{:#?}", dp);
}

fn calc_dp(n: usize, a: i32, b: i32) -> Vec<Vec<Player>> {
    // dp[a][b] := 山に a 個の石が積まれている状態でターンプレイヤーが　from_bin(b) の時に両者が最善を尽くすとどちらが勝つか
    let mut dp = vec![vec![A; 2]; n+1];

    for i in 0..=n {
        for j in 0..=1 {
            let turn_player = Player::from_bin(j);
            let other_player = turn_player.other();
            if i < (a as usize) && i < (b as usize) {
                dp[i][j] = other_player;
            } else {
                // 取れる２択のうち、自分が勝てる方法があるなら勝ち、そうでないなら負け
                if i >= (a as usize) && dp[i- (a as usize)][other_player.to_bin()] == turn_player {
                    // a個とって勝てるなら勝ち
                    dp[i][j] = turn_player;
                } else if i >= (b as usize) && dp[i - (b as usize)][other_player.to_bin()] == turn_player {
                    // b個とって勝てるなら勝ち
                    dp[i][j] = turn_player;
                } else {
                    // a個とってもb個とっても勝てないなら負け
                    dp[i][j] = other_player;
                }
            }
        }
    }

    dp
}
