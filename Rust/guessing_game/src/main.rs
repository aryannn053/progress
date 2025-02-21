use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut cnt: i32 = 0;
    
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("Failed to read as an itneger");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct, You guessed the number in a total of {} attempts", cnt);
                break;
            },
        }
        cnt += 1;
    }
}