use std::io;

fn main() {
    println!("--------------");
    println!("What Your Name?");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Are You Stupid?");
    let mut stupid = String::new();

    io::stdin().read_line(&mut stupid).expect("Failed to read line");

    prints(name, stupid);
}

fn prints(nm: String, st: String) {
    println!("-------------------\nYour Name : {}\nYou Are Stupid : {}", nm, st);
}