#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main(){
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {
        age,
        name
    };

    println!("{:#?}", peter);

    let point: Point = Point { x:10.3, y: 0.4 };
    println!("print coordinate: ({}, {})", point.x, point.y);

    let bottom_right = Point {x : 5.2, ..point};
    println!("{:?}", bottom_right);

    let Point {x: left_edge, y: top_edge} = point;
    let _rectengle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right,
    };
    println!("{:?}", _rectengle);

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;

    println!("pair contains {} and {}", integer, decimal);
}