//Mind The Gap (by Donnamae on edabit.com)

//A number is gapful if it is at least 3 digits long and is divisible by the number formed by stringing the first and last numbers together. 
//The smallest number that fits this description is 100. First digit is 1, last digit is 0, forming 10, which is a factor of 100. Therefore, 100 is gapful.

//*The original challenge has the printed value being the closest gap number. For simplicity's sake I
//will have it print the next gap number instead

use std::io;

fn main() {
    loop {
    let mut num = String::new();
    
    io::stdin().read_line(&mut num)
	    .expect("Failed to read line");
	
	let num: u32 = num.trim().parse()
	    .expect("Failed to read line");

    let mut gap = num;

    if num < 100 {
        println!("The number should be more than 100");

    } else {
        'not: loop {
            let string: String = gap.to_string();
            let arr: Vec<char> = string.chars().collect(); 
            let test = arr[0];
            let test2 = arr[arr.len() - 1];
            let mut test: String = test.to_string();
            test.push(test2);
            let test = test.parse::<u32>().unwrap();
        
            if gap % test == 0 {
                println!("{} -> Gapful: {}", num, gap);
                break;
        
            } else {
                gap += 1;
                continue 'not
            }
        }
    }
}
}