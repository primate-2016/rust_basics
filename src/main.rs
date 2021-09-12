use std::io; // a limited number of libraries are imported in the "prelude", import others from Standard and other libraries
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // :: indicates 'new' is a function of the String type

        io::stdin()
            .read_line(&mut guess)
            // read_line puts what the user typed into 'guess' and also returned a value of type io::Result
            // io::Result can be either 'Ok' or 'Err'. 'Err' contains info about the failured.
            // Result types encode error handling info
            // expect will handle and error and cause the program to crash if an error is returned
            .expect("Failed to read line");
        
        // create a shadow of original guess var, useful to change type
        // parse method parses a string into a number - here a u32 (32-bit int)
        // if parse returns an error then handle it (i.e. if input is not is not a number)
        // we didn't set secret_number to a number type but Rust will infer it
        // based on match and compare below to u32 type for guess shadow
        let guess: u32 = match guess.trim().parse() {
            // if type in Ok result is number
            Ok(num) => num,
            Err(_) => continue, // go rounf the loop again, don't crash the program
        };
        
        println!("You guessed: {}", guess);

        // compare the guess to the secret number using cmp lib
        // match expression is made up of "arms"
        // an arm consists of a pattern and what should happen if the pattern matches
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
