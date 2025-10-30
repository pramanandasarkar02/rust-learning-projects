use std::io;
fn main() {
    let mut exp = String::new();

    println!("Enter equation to solve\n\tAvaliable command and symbol +, -, *, /, (, )");
    print!("Enter x = ");
    io::stdin()
        .read_line(&mut exp)
        .expect("Failed to read line");

    println!("Entered equation {}", exp);
}
