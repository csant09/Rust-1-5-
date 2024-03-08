use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
fn main(){

    // //Guessing Game
    // let mut count=0;
    // println!("This is a guessing game.");
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // loop {
    //     count+=1;
    //     println!("Guess a number.");
    //    let mut guess = String::new();
    //     io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read the input.");
    //     let guess:u32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     match guess.cmp(&secret_number){
    //         Ordering::Equal => {
    //             print!("You guessed correct.\n");
    //             println!("It took you {count} tries.\n");
    //             break;
    //         },
    //         Ordering::Greater => print!("Too big.\n"),
    //         Ordering::Less => print!("Too small.\n")
    //     }
    // }

    // //Inner Shadowing
    // let x = 10;
    // println!("Initial value of x: {x}");
    // let x = x+1;
    // println!("Updated value of x: {}",x);
    // {
    //     let x = x*x;
    //     println!("Inner scope value of x: {}",x);
    // }
    // let x = x+1;
    // println!("Outer scope value of x: {x}");

    // let spaces = "     ";
    // println!("Spaces:'{spaces}'");
    // let spaces = spaces.len();
    // println!("Space Count:{spaces}");

    // //Tuple
    //let tup = ('C',21, "Tinkune",3.0);
    
    // //Method 1 of accessing tuple
    // let(a,b,c,d) = tup;
    // println!(" {c}"); // you might get an error asking to add _ in unused variables.
    
    // //Method 2 of accessing tuple
    // println!("{{tup.0}}"); //double braces had to be used to escape an error caused by .
    // let completed_year = tup.3;
    // println!("{completed_year}");

    // //Arrays
    // let a = [1,2,3,4,5];
    // let b = [10;10]; //contains 10 small value '10'
    // let first = a[3];
    // let second= b[8];
    // println!("{first}");
    // println!("{second}");

    // //Function
    // let ret:i32 = abc();
    // println!("{ret}");

    let mut n = String::new();
    println!("Enter a number.");
    io::stdin().read_line(&mut n).expect("Failed to read input.");
    let n = n.trim().parse().expect("Input a number.");
    let n:i32 = self_multiply(n);
    println!("{n}");




}
// fn abc()->i32{
//     5*6
// }
 fn self_multiply(a:i32)->i32{
    a*a
}