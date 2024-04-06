use proconio::input;

// モノイド(S, op)を表現するトレイト
trait Monoid {
    // 単位元を返すメソッド
    // 単位元 e ∈ S は、任意の x ∈ S に対して、 e op x = x かつ x op e = x を満たす
    fn id() -> Self;

    // S上の二項演算を行うメソッド
    fn op(&self, other: &Self) -> Self;
}

// Segment Tree
// ref. https://tsutaj.hatenablog.com/entry/2017/03/29/204841
// ref. https://hcpc-hokudai.github.io/archive/structure_segtree_001.pdf
struct SegmentTree<T: Monoid> {
    // 内部的にもつ配列は2分木を模していて、その最下段の要素数
    // 対象となる配列の要素数以上となる最小の２べきの数にする
    n: usize,
    // 内部的にもつ配列
    // 1-indexedで持つ
    node: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Monoid + Clone + Copy,
{
    // 初期化
    // v は 0-indexed
    fn init(v: &Vec<T>) -> SegmentTree<T> {
        let len = v.len();
        let n = len.next_power_of_two();

        // node を作成する
        // 最下段のノード数が n(=2^k) の二分木を模した配列なので、要素数は 1 + 2 + ... + 2^k = 2^(k+1) - 1 + 1 (= 2^(k+1))
        // 1-indexed なので要素数が +1 されている
        let mut node = vec![Monoid::id(); 2 * n];
        // 最下段以外の nodeの数は 2^(k+1) - 1 - 2^k = 2^k - 1 なので、
        // 1-indexedな node[] の添え字は 1 ~ 2^k - 1 を使う
        // したがって、 v[i] に対応する node[] の位置は 2^k(=n) + i (v[] は 0-indexed)
        for i in 0..len {
            node[n + i] = v[i].clone();
        }
        for i in (1..=n - 1).rev() {
            node[i] = node[2 * i].op(&node[2 * i + 1]);
        }

        SegmentTree { n, node }
    }

    //　元の配列 vにおける v[i] を T に変更する
    fn update(&mut self, i: usize, val: T) {
        // v[i] に対応するのは　node[n + i]
        let mut cur = self.n + i;
        self.node[cur] = val;

        // 最下段のノードを上に伝播させる
        loop {
            // 親 nodeの添え字を得る
            cur = cur / 2;
            if cur < 1 {
                break;
            }

            self.node[cur] = self.node[2 * cur].op(&self.node[2 * cur + 1]);
        }
    }

    // 元の配列　vと vの添え字での半開区間 [l, r) に対して op(v[l], v[l+1], ..., v[r-1]) を求める
    // op は二項演算子であるが、結合法則を満たすので、自然に拡張される n項演算子として定義されるものを用いている
    fn fold(&self, l: usize, r: usize) -> T {
        // 非再帰で fold するので、直観としては両端の添え字（l, r）を下から　nodeを見て被覆していく
        // 半開区間なので lは自身を含み、rは自身を含まないことに注意
        // - l側の時
        //      自分が親から見て左側の子供(添え字が偶数)である場合は親 node の方で集計して貰えばよく
        //      自分が親から見て右側の子供（添え字が奇数）である場合は親 node の方で集計すると兄弟まで集計対象に入って困るので今集計しないといけないのが注意点
        // - r側の時
        //      そもそもの注意点として、これが表現しているのは「添え字rが指す区間はギリギリ被覆対象に含まれない」、つまり、
        //      自分の左の nodeからちょうど被覆対象になっていることだという意識を持つとわかりやすい
        //      自分が親から見て左側の子供である場合は、自身は被覆対象ではないので /2 して親ノードに移る（この操作の前後で、　rの持つ「添え字rが指す区間はギリギリ被覆対象に含まれない」という意味は保存されている）
        //      自分が親かr見て右側の子供である場合は、自身の兄弟が集計対象なので node[r-1] を集計してから親に移る（ただ親に移るだけだと node[r-1]が集計から漏れてしまう。また rの持つ意味も変わらない）

        // Monoidの二項演算が交換法則を満たさない場合にも対応している
        let mut from_left: T = Monoid::id();
        let mut from_right: T = Monoid::id();

        let mut l = l + self.n;
        let mut r = r + self.n;

        while l < r {
            if l & 1 != 0 {
                from_left = from_left.op(&self.node[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                from_right = self.node[r].op(&from_right);
            }
            l = l >> 1;
            r = r >> 1;
        }

        from_left.op(&from_right)
    }
}

impl Monoid for usize {
    fn id() -> Self {
        0
    }

    fn op(&self, other: &Self) -> Self {
        self + other
    }
}

enum Query {
    Update(usize, usize),
    Fold(usize, usize),
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut segtree = SegmentTree::init(&vec![0; n]);
    for _ in 0..q {
        proc_query(&mut segtree);
    }
}

fn proc_query(segtree: &mut SegmentTree<usize>) {
    match read_query() {
        Query::Update(pos, x) => segtree.update(pos - 1, x),
        Query::Fold(l, r) => println!("{}", segtree.fold(l - 1, r - 1)),
    }
}

fn read_query() -> Query {
    input! { c: usize }
    match c {
        1 => {
            input! { pos: usize, x: usize }
            Query::Update(pos, x)
        }
        2 => {
            input! { left: usize, right: usize }
            Query::Fold(left, right)
        }
        _ => unreachable!(),
    }
}
