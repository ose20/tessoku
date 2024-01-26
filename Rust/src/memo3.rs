use std::ops::Add;

struct NoCopy {}

impl Add for &NoCopy {
    type Output = NoCopy;
    fn add(self, _rhs: &NoCopy) -> NoCopy {
        NoCopy {}
    }
}
fn main()
{
    let a = Vec::<NoCopy>::new();
    let b = Vec::<NoCopy>::new();
    let c = NoCopy {};
    let _cb: Vec<_> = a.iter().flat_map(|ai| b.iter().map(|bi| &c + bi)).collect();
    let _ab: Vec<_> = a.iter().flat_map(|ai| b.iter().map(move |bi| ai + bi)).collect();
    let _ab: Vec<_> = a.iter().map(|ai| b.iter().map(|bi| bi + &a[0])).collect();

}

// mapの返り値 MapもIteratorなので、Itemにはlifetimeがあるはず。なので、bのIteratorなので、bのitemのlifetimeより小さくないといけないけど、aiはそれよりでかい
// でもこれはcやa[0]が反例になってないか？

// 指定がないときは、返り値のlifetimeは引数と同じになるはず
// biのlifetimeはbと同じなので、aと同じだし、cも同じ、でもaiは厳密に狭い

// Rust language references:
//   https://www.rust-lang.org/
