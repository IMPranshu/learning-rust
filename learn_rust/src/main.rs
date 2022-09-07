use std::io;

fn main() {
    // converting string to integer

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");

    let int_in: f64 = input.trim().parse().unwrap();// trim function removes the special char at the end of each string which is bacically invisible when we press enter and go to thte newline.
    // parse function takes the result from input.trim and converts the string into integer.
    // unwrap takes the valid integer result and then unwrap it and stores it as the actual integer value.

    println!("{}", int_in + 2.0);



}

