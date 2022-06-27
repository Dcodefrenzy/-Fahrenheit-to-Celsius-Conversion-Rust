use std::io;
fn main() {
   // println!("Hello, world!");
    println!("Do you want to start Y/N:");
    let mut start = String::new();

    io::stdin()
        .read_line(&mut start)
            .expect("Crashed!!!!!!!!");

    println!("Answer: {}", start);



}
