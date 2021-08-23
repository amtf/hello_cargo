fn main() {
    let num = None;

    match num {
        None => (),
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
    }
}