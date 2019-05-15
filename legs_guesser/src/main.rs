//Checks how many legs are present when there is a certain number of chickens, cows, and pigs
//Inspired by https://edabit.com/challenge/8Qg78sf5SNDEANKti

use std::io;

fn main() {
    'again: loop {
        println!("How many legs if we have Chickens, Cows, and Pigs?");
        println!("How many chickens, cows, and pigs?");

        let chickens = inputs();
        let cows = inputs();
        let pigs = inputs();

        let chlegs = chickens * 2;
        let colegs = cows * 4;
        let plegs = pigs * 4;

        let total = chlegs + colegs + plegs;

        println!("{} chickens = {} legs", chickens, chlegs);
        println!("{} cows = {} legs", cows, colegs);
        println!("{} pigs = {} legs", pigs, plegs);
        println!("{} legs in total!", total);

        println!("Try Again? (1 = yes, 2 = no)");

        'choose: loop {
            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            let answer: u64 = answer.trim().parse().expect("Failed to read line");

            if answer == 1 {
                continue 'again;
            } else if answer == 2 {
                break;
            } else {
                println!("Choose only 1 or 2");
                continue 'choose;
            }
        }
        break;
    }
}

fn inputs() -> u64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u64 = input.trim().parse().expect("Failed to read line");
    return input;
}
