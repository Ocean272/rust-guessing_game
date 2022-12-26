use std::io; // this defines a set of items in its RUST standard library. 
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut refers to mutatable variables.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Rust will insist coder to write "error-handling" code as its in io standard input.

        
        // Shadowing allow reuse the 'guess' variable.    
        let guess: u32 = match guess.trim().parse(){ //here it switch from expect call to match expression to move from
            //.expect("Please type a number")          crashing on an error to handling the error.
            Ok(num) => num, 
            // if parse is able to successfully turn the string into a number, it will return an Ok value
            // hence, match the first arm's pattern and the match expression will just return the num value.
            // The value will then end up right where we want it in the new guess variable we're creating. 
            Err(_) => continue,
            // if parse is not able to turn the string into a number, it will return an Err value.  Since the 
            // Err value does not match the Ok(num) pattern in the first match arm, it does match the Err(_) pattern
            // in the second arm, The underscore is a catchall value. So if the program execute the second arm's code 'continue',
            // the program will go into the next iteration of the loop and ask for another guess. Therefore,
            // effectively, the program ignores all errors that parse might encounter.
        };


        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
