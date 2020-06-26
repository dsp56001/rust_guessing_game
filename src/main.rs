use std::io;
use std::cmp::Ordering;
use rand::Rng;
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut min = 1;
    let mut max = 100;
    loop {
        
        println!("Please input your guess from {} to {}", min, max);

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => { 
                print_ferris("Too small!");
                min = guess;
            }
            Ordering::Greater => 
            {
                //println!("Too big!");
                print_ferris("Too big!");
                max = guess;
            }
            Ordering::Equal => 
            {   
                //println!("You win!");
                
                print_ferris("You win!");
                break;
            }
        
        }
    }
}

fn print_ferris(msg: &str) {
    //println!("the message is {}", msg);
    let stdout  = stdout();
    let message = String::from(msg);
    let width   = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

