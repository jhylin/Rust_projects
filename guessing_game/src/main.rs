// io = input/output library
// io library comes from standard (std) library
// Rust has set of items (= prelude) defined in the std library
// that it brings into the scope of every program
// If a type needed is not in the prelude, need to bring that type
// into the scope explicitly using "use" statement
// Essentially std::io allows to accept user inputs
use std::io;

// Rng trait defines methods that random number generators implement
// This trait must be in scope for us to use these methods
use rand::Rng;

// Ordering type is another enum and has the variants Less, Greater and Equal
// Taking in account the 3 outcomes that are possible when comparing two values
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // Call rand::thread_rng function that gives the particular random no. generator 
    // one that is local to the current thread of execution and seeded by the operating system
    // then call gen_range method on the random no. generator
    // this method is defined by the Rng trait with the use rand::Rng statement
    // gen_range method takes a range expression as an argument 
    // and generates a random number in the range
    // the kind of range expression being used here takes the form of start..=end
    // so need to specify 1..=100 to request a no. between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Remove comment for line below to test program if needed
    // println!("The secret number is: {secret_number}");
    
    // give users more chances at guessing the number by using a loop
    loop {
        println!("Please input your guess.");

        // Create a variable to store user input
        // In Rust, variables are immutable by default
        // If we give the variable a value, the value won't change
        // To make a variable mutable, we add "mut" before the variable name
        // On the right of the = sign is where the value is bound to "guess",
        // which is the result of calling String::new, a function that returns 
        // a new instance of a String
        // The :: syntax indicates that "new" is an associated function of the String type
        // An associated function is a function that's implemented ona type, in this case, String
        // The "new" function creates a new empty string
        // In essence, this line has created a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        // Call "stdin" function from the io module to handle user input
        io::stdin()
            // Calls the read_line method on the standard input handle to get input from user
            // Passing &mut guess as argument to read_line to tell it what string to store the user input in
            // read_line will take whatever the user types into standard input 
            // and append that into a string (without overwriting its contents)
            // So we will then pass that string as an argument
            // The strong argument needs to be mutable so that the method can change the string's content
            // & indicates this argument is a reference 
            // -> gives a way to let multiple parts of your code to access one piece of data 
            // without needing to copy that data into memory multple times
            // References are a complex feature in Rust and is a major advantage as it's safe and easy
            // References are immutable by default, so need to write &mut to make it mutable
            .read_line(&mut guess)
            
            // read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value
            // Result is an enumeration (enum), a type that can be in one of multiple possible states
            // Each possible state = variant
            // The purpose of Result types is to encode error-handling information
            // Result's variants are Ok and Err
            // Ok variant = the operation was successful
            // Inside Ok is the successfully generated value
            // Err variant = operation failed and contains info about how/why the operation failed
            // Values of the Result type have methods defined on them
            // An instance of Result has an expcet method that we can call
            // If this instance of Result is an Err value, 
            // expect will cause the program to crash and display the message that you passed as an arguemnt to expect
            // If this instance of Result is an Ok value, 
            // expect will take the return value that Ok is holding and return just that value so you can use it
            // In this case, the value is the number of byres in the user's input
            .expect("Failed to read line");
    
        // Rust allows to shadow the previous value of guess with a new one
        // Shadowing lets us reuse the guess variable name, 
        // rather than forcing us to create 2 unique variables such as guess_str and guess
        // This feature is often used when wanting to convert a value from one type into another type
        // Bind this new variable to the expression guess.trim().parse()
        // The guess in the expression refers to the original guess variable that contained the input as a string
        // The trim method on a String instance will eliminate any whitespace at the beginning and end, 
        // which we must do to be able to compare the string to the u32, which can only contain numerical data
        // The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string
        // The trim method eliminates \n or \r\n resulting in jus the number
        // The parse method on strings converts a string to another type
        // We need to tell Rust the exact number type we want by using let guess: u32
        // The colon after guess tells Rust we'll annotate the variable's type
        // u32 = unsigned 32-bit integer (good default choice for small +ve number)
        
        // Switch from expect call to match expression to move from crashing on an error to handing the error
        // Recall parse returns a Result type which is an enum that has variants Ok and Err
        // If parse is able to turn the string into a number
        // It'll return an Ok value that contains the number, which will end up in the new guess variable we're creating
        // If parse is NOT able to turn the string into a number, it'll reutrn an Err value
        // The underscore __ is a catchall value
        // The program will execute the second arm's code, continue, which tells the program to go to the next iteration of the loop and ask for another guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Prints the string that now contains the user's input
        // The {} is a placeholder
        // Can print more than one value using {}
        // e.g. let x = 5;
        // let y = 10;
        // println!("x = {} and y = {}", x, y); 
        println!("You guessed: {guess}");
    
        // cmp method compares two values and can be called on anything that can be compared
        // It takes a reference to whatever you want to compare with e.g. comparing the guess to the secret_number
        // It returns a variant of the Ordering enum we brought into scope with the use statement
        // then use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in the guess and secret_number
        // A match expression is made up of arms
        // Arm consists of a pattern to match against and the code that should be run if the value given to match fits that arm's pattern
        // Rust takes the value given to matchh and looks through each arm's pattern in turn
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Make the program exits the loop when the user guesses the number correctly
                break;
            }

        }
        
    }
    
}
// Run and test program via terminal using command: cargo run