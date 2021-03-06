//Own version of Guessing Game activity from Rust Language documentation (https://doc.rust-lang.org/book/)

extern crate rand;

use std::io;
use rand::Rng;

fn main() {
	
'right: loop {
    let mut random = rand::thread_rng().gen_range(1, 11);
	
'wrong: loop {

	println!("Input Guess!");
	
	let mut answer = String::new();
	
	io::stdin().read_line(&mut answer)
	    .expect("Failed to read line");
	
	let answer: u32 = answer.trim().parse()
	    .expect("Failed to read line");
	
	if answer < 1 || answer > 10 {
    println!("Please print a number from 1-10");

    } else { 	
	
	println!("Your Guess: {}", answer);
	
	    if answer == random {
		    println!("you win!");
			continue 'right
			
		} else if answer > random {
            println!("too big");
            continue 'wrong

        } else if answer < random {
            println!("too small");
            continue 'wrong		
		}
	}
    }
}
}
