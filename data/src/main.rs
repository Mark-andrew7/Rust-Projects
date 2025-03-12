use std::io::{stdin,stdout,Write};

fn main() {
    let mut s=String::new();
    print!("Enter some text: ")
    let _=stdout().flush();
    let num = stdin().parse(&mut s).expect("Failed to parse string to integer");
    println!(num);
}
