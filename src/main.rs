use std::io;
fn main() {
   // println!("Hello, world!");

    println!("!!!!!!!This is a temptature converting app!!!!!!!!!!");
    println!("Do you want to start  Y/N:");
    let mut start = String::new();

    io::stdin()
        .read_line(&mut start)
            .expect("Crashed!!!!!!!!");

            start_game(start);


}

fn start_game(start: String) {
    let start : char =  start.trim().parse().expect("Should either be Y or N");
     match start {
                     'y'|'Y' =>  conversion_to(),
                     _ => end_conversion(),
    };

}


fn conversion_to() {
    println!("Do you want to convert to F to C (Enter F) or C to F (Enter C):");
    let mut conversion = String::new();

    io::stdin()
        .read_line(&mut conversion)
            .expect("Crashed!!!!!!!!");

            let conversion:char = conversion.trim()
                                        .parse()
                                        .expect("Please enter a cooresponding answer");

            match conversion {
                'F'|'f' =>conversion_to_celsius(),
                'C'|'c' => conversion_to_fahrenheit(),
                _=>restart()
            }
}


fn conversion_to_fahrenheit() {
    println!("!!!!!!!!!!
    The Fahrenheit temperature 
    scale is named for German physicist Daniel Gabriel Fahrenheit and is 
    the measurement of temperature commonly used by the United States 
    (and its associated territories) and by several nations in the Caribbean. 
    On the Fahrenheit scale, water freezes at 32°F and boils at 212°F (at sea level)
    !!!!!!!!!!");
    println!("Enter the number you want to convert:");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
            .expect("Crashed!!!!!!!!");
    let num : i32= num.trim()
                        .parse()
                        .expect("The value you entered is Not a number Please enter a number");
            let answer = (num * 9/5) + 32;
            println!("Your Answer is : {} F", answer);
}

fn conversion_to_celsius() {
    println!("!!!!!!!!!!
    The Celsius temperature scale—originally called centigrade and later renamed for Swedish astronomer 
    Anders Celsius—is used almost everywhere else in the world. On the Celsius scale,
     water freezes at 0°C and boils at 100°C (at sea level).
    !!!!!!!!!!");
    
    println!("Enter the number you want to convert:");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
            .expect("Crashed!!!!!!!!");
    let num : i32= num.trim()
                        .parse()
                        .expect("The value you entered is Not a number Please enter a number");
            let answer = (num - 32) * 5/9;
            println!("Your Answer is : {} C", answer);
}


fn end_conversion() {
    println!("Please Pick (Y) to convert your temprature");
}

fn restart() {
    println!("please restart the processs.");
}
//Start
//Name
//What you want to convert
//Convert
//Result

//Do you want to convert again.
