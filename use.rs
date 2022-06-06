#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

#[derive(Debug)]
enum Work {
    Civilian,
    Solider,
}

fn main(){
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    println!("{:?}", status);

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians Work!"),
        Solider => println!("Soliders fight")
    }

    println!("Hello")
}