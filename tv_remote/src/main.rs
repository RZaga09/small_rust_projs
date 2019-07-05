//HISTORICAL TV REMOTE CONTROL
//from UCF Local Contest — September 1, 2018
//link to questions - https://lpc.ucfprogrammingteam.org/localFiles/local2018Problems.pdf

use std::io;

fn main() {

	println!("Some of the 10 buttons on your TV remote labeled 0-9 are broken!");
	println!("The channel up and down buttons are working though.");
	println!("See how many up or down presses you need to reach your desired TV channel!");

	loop {
		println!("How many buttons are broken?");

		let mut broke = String::new();
	
		io::stdin().read_line(&mut broke)
	    	.expect("Failed to read line");
	
		let broke: i32 = broke.trim().parse()
	    	.expect("Failed to read line");

	   	let mut num: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
		let mut num2: Vec<i32> = vec![];

		if broke < 1 || broke > 9 {
			println!("Input a number between 1 and 9");

		} else {
			println!("What buttons are broken");
	
			for i in 0..broke {

			'again: loop {
	
			let mut but = String::new();

			io::stdin().read_line(&mut but)
    			.expect("Failed to read line");
		
			let but: i32 = but.trim().parse()
    			.expect("Failed to read line");

    		if but < 0 || but > 9 {
    			println!("Input a number between 1 and 9");
    			continue 'again
    		}

    		if num2.contains(&but) {
    			println!("Choose a number that wasn't already broken");
    			continue 'again
    		}

    		if num.contains(&but) {
    			num.retain(|&x| x != but);
    			num2.push(but);
    		}
    		break;
    	}

		}
		loop {
		println!("Which channel do you want to reach?");

		let mut target = String::new();
	
		io::stdin().read_line(&mut target)
	    	.expect("Failed to read line");
	
		let target: i32 = target.trim().parse()
	    	.expect("Failed to read line");

	    if target < 0 || target > 999 {
	    	println!("Theres only channels 0-999");

	    } else {
	    	loop {
	    		let mut num3: Vec<i32> = vec![];
	    		let mut base = target;
	    		let mut is_in = true;
	    		let string: String = base.to_string();
	    		let arr: Vec<char> = string.chars().collect();

	    		for i in arr {
	    			let i: String = i.to_string();
	    			let i = i.parse::<i32>().unwrap();
	    			num3.push(i);
	    		}

	    		for i in num3 {
	    			if num.contains(&i) {
	    				is_in = true;
	    			} else {
	    				is_in = false;
	    				break;
	    			}
	    		}

	    		if is_in == true {
	    			println!("0 clicks to {}", target);
	    			break;

	    		} else {
	    			let mut y = 0;
	    			let mut z = -1;

	    			loop {
	    			let c = y * z;
	    			base = base + c;
	    			let mut num3: Vec<i32> = vec![];
		    		let mut is_in = true;
	    			let string: String = base.to_string();
	    			let arr: Vec<char> = string.chars().collect();

	    			for i in arr {
	    				let i: String = i.to_string();
	    				let i = i.parse::<i32>().unwrap();
	    				num3.push(i);
	    			}

	    			for i in num3 {
	    				if num.contains(&i) {
	    					is_in = true;
	    				} else {
	    					is_in = false;
	    					break;
	    				}
	    			}

	    			if is_in == true {
	    				let mut j = (base - target);
	    				j = j.abs();
	    				println!("{} clicks to {}", j, target);
	    				break;
	    			} else {
	    				y += 1;
	    				z = -z;
	    			}
	    		} 
	    		break;
	    		}
	    	}
	    }
	    break;
		}
		}
	}
}