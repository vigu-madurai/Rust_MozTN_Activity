/**
**
Basic Rust Code to perform addition, subtraction, multiplication & division of two numbers from the end users
**
*/

//standard input output library
use std::io;	

//main code works here!
fn main() {
    
    println!("Welcome to perform Addition, Subtraction, Multiplication and Division of two numbers!");
    
    //variable declarations
    let mut n1 = String::new();
    let mut n2 = String::new();

    //INPUT 1
    println!("Enter a number 1: ");
    //Reading number 1 from the end user as a string!
    io::stdin().read_line(&mut n1)
    	.expect("Failed Attempt!");
    //Conversion of string to signed integer!
    let num1 :i64= n1.trim().parse().ok()
    	.expect("Enter a number only");
    
    //INPUT 2
    println!("Enter a number 2: ");
    //Reading number 2 from the end user as a string!
    io::stdin().read_line(&mut n2)
    	.expect("Failed Attempt!");
    //Conversion of string to signed integer!
    let num2 :i64= n2.trim().parse().ok()
    	.expect("Enter a number only");
    
    //Result goes here!
    println!("ADDITION = {}",num1+num2);
    println!("SUBTRACTION = {}",num1-num2);
    println!("MULTIPLICATION = {}",num1*num2);
    println!("DIVISION = {}",num1/num2);
}
