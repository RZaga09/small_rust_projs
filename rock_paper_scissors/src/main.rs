//Rock, Paper, Scissors in Rust

extern crate rand;

use std::io;
use rand::Rng;

fn main() {

	loop {

		println!("0 = Rock");
		println!("1 = Paper");
		println!("2 = Scissors");

		let comp_num = rand::thread_rng().gen_range(0, 3);

		let mut pl_num = String::new();

		io::stdin().read_line(&mut pl_num)
		.expect("Failed to read line");

		let pl_num: u32 = pl_num.trim().parse()
		.expect("Failed to read line");

		println!("Computer: {}", comp_num);
		println!("Player: {}", pl_num);

		if pl_num > 2 {
			println!("Type num from 0-2");
		}

		if comp_num == pl_num {
			println!("Draw");
		} else if pl_num == 0 {
			if comp_num == 1 {
				println!("You Lose");
			} else {
				println!("You Win");
			}
		} else if pl_num == 1 {
			if comp_num == 0 {
				println!("You Win");
			} else {
				println!("You Lose");
			}
		} else if pl_num == 2 {
			if comp_num == 1 {
				println!("You Win");
			} else {
				println!("You Lose");
			}
		}
		println!(" ");
	}
}