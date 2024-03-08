use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    let mut count=0;
    println!("This is a guessing game.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        count+=1;
        println!("Guess a number.");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input.");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                print!("You guessed correct.\n");
                println!("It took you {count} tries.\n");
                break;
            },
            Ordering::Greater => print!("Too big.\n"),
            Ordering::Less => print!("Too small.\n")
        }
    }
}
