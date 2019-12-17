use std::io;
use std::convert::TryInto;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let length:u32 = (input.len()-1).try_into().unwrap();
    let mut sum:u32 = 0;
    for (i, c) in input.chars().enumerate(){
      if i < length.try_into().unwrap(){
        sum = sum + (c as u32 - 48).pow(length);
      }
    }
    let n:u32 = input.trim().parse().unwrap();
    if n == sum{
      println!("It's an Armstrong number!");
    }
    else{
      println!("It's not an Armstrong number.");
    }
}

//Read input from stdin, emumerate through input, find sum and compare to determine if it's an armstrong number.
