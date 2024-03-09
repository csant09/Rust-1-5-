//use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
fn main(){
    // //Guessing Game
    // let mut count: i32=0;
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
    // let tup = ('C',21, "Tinkune",3.0);
    
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

    // let mut n = String::new();
    // println!("Enter a number.");
    // io::stdin().read_line(&mut n).expect("Failed to read input.");
    // let n = n.trim().parse().expect("Input a number.");
    // let n:i32 = self_multiply(n);
    // println!("{n}");

    
    // //Control Statements

    // let result1 = if 5>7 {2} else {3};
    // let mut counter = 0;
    // let result2 = loop {
    //     counter+=1;
    //     if counter == 10 {
    //         break counter * 8;
    //     }
    // };
    // println!("Result1:{result1}");
    // println!("Result2:{result2}");


    // //Multiple Loops
    // let mut outer_count = 0;
    // 'outer_loop : loop {
    //     let mut inner_count = 5;
    //     print!("Outer count: {outer_count}  ");
    //     print!("Inner count: ");
    //     'inner_loop: loop {
    //         print!("({inner_count})");
    //         if inner_count==0 {
    //             print!("\n");
    //             break 'inner_loop;
    //         }
    //         if outer_count==5{
    //             break 'outer_loop;
    //         }
    //         inner_count-=1;
    //     }
    //     outer_count+=1;
    // }


    // //While
    //  let arr =[2,4,6,8,10];
    // let mut index = 0;
    // while index!=5 {
    //     println!("Index {}={}",index,arr[index]);
    //     index+=1;
    // }

    // for element in arr {
    //     print!("{element} ");
    // }

    // for number in 1..=5{ //= diyena vaney chai 5 include hudaina. 1 to 4 matra hunxa.
    //     println!("{number}" );
    // }


    // //Stack Representation of string
    // let s1 = "hello"; 
    // //even if we declare s1 as mutable, we can't perform push operation because it isn't heap.
    // println!("{}",s1);

    // //Heap Representaion of string
    // let mut s2 = String::from("hello");
    // s2.push_str(" world");
    // println!("{}",s2);

    //Shallow Copy or Move
    //Shallow copy means that instead of copying the actual data "hello world" to both heaps c1 and c2,
    //only the pointer is copied. i.e. only one stack data, and two heaps poinitng to the same data.
    // let c1 = String::from("hello world");
    // let c2 = c1;
    // //println!("{}",c1); //This line will generate an error because c1 has already been moved to c2
                            // and no longer exists to prevent double dropping(freeing) 
    // println!("{}",c2);


    // //Deep Copy
    // //Unlike shallow copy, two separate stacks are created aka clones.
    // let d1 = String::from("hello world");
    // let d2 = d1.clone();
    // println!("{} {}",d1,d2);


    // //Variation of Move also occurs in the function calls.
    // let e = String::from("Csant");
    // move_string(e);
    // //println!("{}",e);  //This will generate an error as e was moved to the function as a parameter.

    // let f = 2;
    // copy_number(f); //This doesn't generate error because integer is a stack type; not heap.
    // println!("{}",f);

    // //How to reuse the data that also needs to be passed in the funtion?
    // //Method 1: Tuples
    // let re=String::from("Csant");
    // let (a,b):(String,usize) = push_and_return(re);
    // println!("The length of {a} is {b}.");

    // //Method 2: References
    // let re = String::from("C Sant Shrestha");
    // let a = calculate_len(&re);
    // println!("The length of {re} is {a}.");
    // //The changes made through references lasts i.e. is present after the function ends.

    // //Mutable and Immutable References
    //let string1 = String::from("Rust");
    // let mut string2 = String::from("Programming");
    // change_immutable(&string1);
    // change_immutable(&string2);
    // //change_mutable(&string1); //This line will cause an error because string1 isn't mutable.
    // //string2 although mutable can be passed as immutable.
    // //This is because immutability has no strict requirements but mutability does.
    // change_mutable(&mut string2);

    // //Multiple mutable and immutable references
    // let r1 = &string1;
    // let r2 = &string1;
    // //Multiple immutable references are permitted but, multiple mutable references isn't possible.
    // let r3 = &string2;
    // let r4=&string2;
    // //let r5 = &mut string2; //This line will cause an error as
    // //string2 can't be referenced as mutable and immutable at the same time.
    // println!("{} {} {} {} ",r1,r2,r3,r4);

    // //However there is an exception when mutable and immutable can co-exist.
    // let r1 = &string2;
    // let r2 = &string2;
    // println!("{r1} {r2} ");
    // let r3 = &mut string2; //This is valid because the scope of r1 and r2 has ended with println!.
    // println!("{r3}");


    // //Slices
    // let name = String::from("Shishant Shrestha");
    // let slice1 = &name[..=5];
    // let slice2 = &name[3..];
    // let slice3 =&name[4..8];
    // println!("{} {} {}",slice1,slice2,slice3);





}
// fn abc()->i32{
//     5*6
// }
//  fn self_multiply(a:i32)->i32{
//     a*a
// }
// fn move_string(e1:String){
//     println!("{}",e1);
// }
// fn copy_number(f1:i32){
//     println!("{}",f1);
// }
// fn push_and_return(mut re:String)->(String,usize){
//     re.push_str(" Shrestha");
//     let length = re.len();
//     (re,length)
// }
// fn calculate_len(re1:&String)->usize{
//     re1.len()
// }
// fn change_mutable(c:&mut String){
//     c.push_str(" hehehe");
//     println!("{c}");
// }
// fn change_immutable(c:&String){
//     println!("{c} hehehe");
// }