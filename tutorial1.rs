use std::io;

fn main() {
    /* Name Input */
    println!("Enter Your Name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Input Failed");

    /* Age Input */
    println!("Enter Your Age");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Input Failed");

    let age:u8 = age.trim().parse().unwrap(); 

    /* Welcome Messages */
    println!("Hello {}Your age is {}", name, age);
}
