use crate::p_square::perfectsquare;
mod p_square;
mod convert;
mod square_root;

fn main() {
    println!("\n **** A program to calculate the minimum number of perfect squares in a number **** \n");
    let num_str = "128"; 
    let (count, perfect_squares) = perfectsquare(num_str);
    println!("The number of perfect squares in the number is: {}", count);
    println!("The perfect squares are: {:?}", perfect_squares);
}