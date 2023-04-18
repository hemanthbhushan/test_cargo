use std::io;
fn main() {
    let _x =  String::new();
    let mut x = String::new();
    println!("enter the value" );

    io::stdin().read_line(&mut x).expect("wrong");
    println!("You guessed: {x}");

}
