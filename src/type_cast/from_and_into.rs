use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn from_and_into() {
    // let num = Number::from(30);
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
