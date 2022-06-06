fn main(){
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    println!("{0} {1} {2}", logical, a_float, an_integer);

    let default_float = 3.0;
    let default_integer = 7;

    let mut infferd_type = 12;
    infferd_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    let mutable = true;
    let mutable = 1;
}   