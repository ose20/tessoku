use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    // 不偏ゲームなので、Grundy数が定義できる
    // 不偏ゲーム: 二人零和有限確定完全情報ゲームのうち、各状態でどちらのプレーヤが手を打つにしても、動かす選択肢の集合が常に等しいゲーム　(by wiki)
    // Grundy数は次のように、状態に対して再帰的に定義される非負整数である
    //  - 遷移できる状態がない時、その状態のGrundy数は0である
    //  - 状態 S から遷移できる状態が S1, ..., Sk である時、それぞれの Grundy 数を G1, ..., Gk が定まる。
    //    この時、集合 {G1, ..., Gk} に含まれない最小の非負整数を S の Grundy 数とする
    // この問題の場合、n個の山を別々のゲームとして考え、それぞれのGrundy数を Gi とすると、
    // ゲームに勝てることと、n個の Grundy 数のXOR和が 0 になることと同値になるらしい（証明してない）
    
    // Grundy数の定義からすぐにわかる補題として、取れる選択肢が n 個しかない時、いかなる状態の Grundy 数も n より大きくはならない（背理法をすればすぐにわかる）

    let max = 100000;
    let grundy = calc_grundy(x, y, max);
    let xor_sum: usize = a.iter().map(|a| grundy[*a]).fold(0, |acc, g| acc ^ g);
    if xor_sum == 0 { println!("Second"); }
    else { println!("First"); }


}

// 石が i 個あるという状態に対する Grundy 数を 0 <= i <= max まで求める
// 取れる操作は x または y 個取り除くこと
fn calc_grundy(x: usize, y: usize, max: usize) -> Vec<usize> {
    // grundy[i] := 1つの山を個別にみたときのゲームの、山に i 個石がある状態の Grundy数
    let mut grundy = vec![0; max+1];

    for i in 0..=max {
        // transitable[g] := 状態 i から遷移可能な状態のうち、Grundy 数が g のものがあるかどうか
        // Grundy数は 2 より大きくならないので、配列の大きさは 0~2 までの 3 つ持っておけば十分
        let mut transitable = vec![false, false, false];
        // x 個とるという操作ができる時、そうした先の状態のGrundy数を記録する
        if i >= x { transitable[grundy[i-x]] = true; }
        // y 個とるという操作ができる時、そうした先の状態のGrundy数を記録する
        if i >= y { transitable[grundy[i-y]] = true; }

        // 遷移先の状態が取りうる Grundy 数を避けながら、最小値を探す
        if !transitable[0] { grundy[i] = 0; }
        else if !transitable[1] { grundy[i] = 1; }
        else if !transitable[2] { grundy[i] = 2; }
        else { unreachable!() }
    }   

    grundy
}
