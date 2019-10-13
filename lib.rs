#[warn(unused_variables)]
use std::io;
// input sign 8-Bit
pub fn inputii8()->i8{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<i8>().expect("cant convert");
  num
}
//Input sing 16-Bit
pub fn inputi16()->i16{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<i16>().expect("cant convert");
  num
}

//Input sing 32-Bit
pub fn inputi32()->i32{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<i32>().expect("cant convert");
  num
}
//Input sing 64-Bit
pub fn inputi64()->i64{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<i64>().expect("cant convert");
  num
}
//Input sing isize-Bit
pub fn inputisize()->isize{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<isize>().expect("cant convert");
  num
}

// input unsign 8-Bit
pub fn inputu8()->u8{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<u8>().expect("cant convert");
  num
}
//Input unsing 16-Bit
pub fn inputu16()->u16{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<u16>().expect("cant convert");
  num
}

//Input unsing 32-Bit
pub fn inputu32()->u32{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<u32>().expect("cant convert");
  num
}
//Input unsing 64-Bit
pub fn inputu64()->u64{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<u64>().expect("cant convert");
  num
}
//Input sing usize-Bit
pub fn inputusize()->usize{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<usize>().expect("cant convert");
  num
}
//Input Float 64-Bit
pub fn inputf64()->f64{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<f64>().expect("cant convert");
  num
}
//Input Float 32-Bit
pub fn inputf32()->f32{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<f32>().expect("cant convert");
  num
}
//Input String
pub fn inputstr()->String{
println!("Enter Value");
let mut input= String::new();
io::stdin().read_line(&mut input).expect("failed to read from stdin");
let num=input.trim().parse::<String>().expect("cant convert");
  num
}