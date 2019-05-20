//Prints the reverse of a printed word
//Inspired by coderbyte.com

use std::io;

fn main() {
    'primary: loop {

    println!("type word!");

    let mut answer = String::new();
	
	io::stdin().read_line(&mut answer)
	    .expect("Failed to read line");

    //puts each char of String into array
    let mut chars: Vec<char> = answer.chars().collect(); 
    
    chars.reverse(); // reverses array

    println!("Reversed Word:");

    for c in chars {
        print!("{}", c);
        }

    println!("     ");
    } 
}
