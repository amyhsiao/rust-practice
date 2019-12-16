use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let y:i32 = input.trim().parse().unwrap();
    if (y%100 != 0 || y % 400 == 0)&& y % 4 == 0{
        println!("It's a leap year!");
    }
    else {
        println!("Not a leap year.");
    }
}
//Used stin to get input and a simple condition to determine if it's a leap year.
