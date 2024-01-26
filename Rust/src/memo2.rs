use std::ops::{Add, Deref};

struct Dummy {}

//
//impl Add for &Dummy {
//    type Output = Dummy;
//    fn add(self, _rhs: &Dummy) -> Dummy {
//        Dummy {}
//    }
//}

impl Add for Dummy {
    type Output = Dummy;
    fn add(self, _rhs: Dummy) -> Dummy {
        Dummy {}
    }
}



fn main() {
    let a = Vec::<Dummy>::new();
    let b = Vec::<Dummy>::new();

    let c = Dummy {};
    let d = Dummy {};
    
}

}