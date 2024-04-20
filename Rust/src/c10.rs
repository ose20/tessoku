use proconio::input;

const MODULO: usize = 1_000_000_007;

fn main() {
    input! {
        w: usize,
    }

    // 1列目は4*3通り
    // 2列目以降を考える(i列目とする)
    // x行y列目のマスを(x,y)と表現する
    // 1行i列目が2行(i-1)列目と同じ場合とそうでない場合に分ける
    //  - 同じ場合: (1,i)は1通り、(2,i)は3通り　=> 3通り
    //  - 違う場合: (1,i)は2通り、(2,i)は2通り => 4通り
    // 2行目以降は独立に考えていい
    //  - なぜならi列目の色を決める時にi-2列目以前のことを考える必要はない
    //  - なぜなら常にi-1列目には2つの異なる色が並んでいるので
    // 答えは 4*3 * 7^(w-1)

    let ans = 12 * mod_pow(7, w - 1) % MODULO;
    println!("{}", ans);
}

// x^y mod modulo
fn mod_pow(x: usize, y: usize) -> usize {
    if y == 0 {
        1
    } else {
        let root = mod_pow(x, y / 2);
        if y % 2 == 0 {
            root * root % MODULO
        } else {
            (root * root % MODULO) * x % MODULO
        }
    }
}
