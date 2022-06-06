static LAGUAGE: &str = "Rust";
const THRESHHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LAGUAGE);
    println!("The threshhold is {}", THRESHHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
}