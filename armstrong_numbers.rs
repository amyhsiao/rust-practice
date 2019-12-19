use std::io;
//use std::convert::TryInto;
fn main() {
    let mut input = String::new();
    println!("Give me a number : ");
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let n:u32 = match input.trim().parse(){
      Ok(n) => {
        println!("Successfully parsed.");
        n
      },
      Err(_) =>{
        println!("Falied to parse.");
        return;
      },
    };
    let mut v = Vec::new();
    let mut temp:u32 = n;
    let mut sum:u32 = 0;
    let mut cnt = 0;
    while temp > 0{
      v.push(temp%10);
      temp = temp / 10;
      cnt = cnt + 1;
    }
    for i in v{
      sum  = sum + (i).pow(cnt);
    }
    //println!("{} {}", n, sum);
    if n == sum{
      println!("It's an Armstrong number!");
    }
    else{
      println!("It's not an Armstrong number.");
    }
}
