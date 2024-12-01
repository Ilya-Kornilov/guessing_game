use std::io;
use std::io::Write; // a lib to stay on the same line
use std::cmp::Ordering; // compares numbers in three variants:
                        // less than, equals to and greater than
use rand::Rng;  // brings "randomizer" into the scope
                // requires a new dependancy in Cargo.toml
                    // after adding a new dependancy
                    // the program has to have "cargo build" AGAIN
use colored::*;    // a new dependancy
                   // pay attention to "...::*" !!!

fn main() {
    println!("Guess the number!\n");

    // a variable to store a randomly generated number
    // in a range from 1 to (including) 100 
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    ///// println!("The secret number is: {}", secret_number);
    
    loop {
        // print! (NOT println!) to stay on the same line
        print!("Please input your guess: "); 
        // additional function to do so
        io::stdout().flush().unwrap();

        // declaring a variable
        // "let" variables are immutable by default
        // to enable changes we need to add "mut" between var and var_name
        let mut guess = String::new();

        // reading user's input and assigning it to the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parsing integer value from the string
            // and assigning it to the varible
            // changing its type = shadowing
        // adding "match" to escape throwing an exeption
        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        ///// println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // pay attention to COMMAs at the end of EACH evaluation
            Ordering::Less => println!("{}", "that was too small".red()),
            Ordering::Greater => println!("{}", "that was too big".red()),
            Ordering::Equal => {
               println!("{}", "And now you win!".green());
               break; 
            },  // also a comma
        }
    }
}
