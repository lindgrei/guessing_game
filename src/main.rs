use rand::Rng;
use std::cmp::Ordering;
use std::io;

//The main method, entry point to every rust program
fn main() {
    //println!() macro to print a message to the screen
    println!("Guess the number");

    // secret number, a 32 byte integer  number that we got from the `rand` crate using the `rand::Rng` trait that gives us `gen_range(range)`
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //This is an interpolated string using the {} notation.
    println!("The Secret Number is : {}", secret_number);

    loop {
        println!("Please input your string");

        //We create a new instance of the String type, `String::new` == ""
        let mut guess = String::new();

        //Here we use io::Stdin to give us a handle over the standard input (the terminal)
        io::stdin()
            .read_line(&mut guess) //This reads whatever was inputted in the terminal and binds the contents to a mutable reference of the
            // guess variable, This results in a Result that has either a string, or an error
            .expect("Failed to read line"); //To Handle the case that the Result has an error in it, we use the expect(msg) to send a message back to the user

        // //Here we want to change the type of the string variable so we `shadow` it by using the same name but with a different type String => u32
        // let guess: u32 = guess
        //     .trim() //We remove all trailing whitespace, /r /n etc... to make the comparison as exact as possible
        //     .parse::<u32>() // We then parse the string with the .parse::<type> to check if there is any number there it also returns a Result with either a u32 or a ParseIntError
        //     .expect("Please type a number!"); //So to handle the ParseIntError, we use a .expect()


        // same type casting, but here instead of making it crash when something other than a number was passed in it just
        //continues and doesn't crash it.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {guess}");

        /*
           This is a match statement, that takes an Ordering that is returned from `guess.cmp()`, the `Ordering` was taken from the `use std::cmp::Ordering` crate
           and it is to compare the guessed number with the secret, generated number, it has arms that it will check one by one.
        */
        match guess.cmp(&secret_number) {
            //In case The guess < secret_number we will execute this arm
            Ordering::Less => println!("Too Small!"),
            //guess > secret_number, this arm
            Ordering::Greater => println!("Too Big"),
            //guess == secret_number, this arm
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}


