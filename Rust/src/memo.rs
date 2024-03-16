use std::fmt::Error;

type MyResult<T> = Result<T, Error>;

fn main() {
    let a: MyResult<i32> = Ok(1);
    let b: MyResult<i32> = Ok(2);

    let mut v: Vec<MyResult<i32>> = Vec::new();
    v.push(a);
    v.push(b);

    let ans: MyResult<Vec<i32>> = v.into_iter().collect();


    println!("{}", a == b);
}



