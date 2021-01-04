fn main() {
    let numers = 42;
    match numers{
        0 => println!("origin"),
        1...3 => println!("all"),
        |5|7|13 => print!("bad"),
        n @ 42=>println!("hahahh",),
        _ => println!("common"),
            }
}
