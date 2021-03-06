fn main(){
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
        object = "the lazydog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{} of {:b} people know binary, the other half doens't", 1, 2);

    println!("{number:>width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");
}
