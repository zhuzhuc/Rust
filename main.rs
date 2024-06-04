use core::num;
use std::{cmp::Ordering, io};// prelude
use rand::Rng;//trait

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);//i32 u32 i64

    //println!("The secert number is {}", secret_number);

    loop {
        println!("please input your guess.");
        // println("please input your guess.");
        //let foo = 1;
        //let bar = foo;//immutable

        //foo = 2; false
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)//io::result ok, error
            .expect("failed to read line");

                //shadow
        let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue, 
        };

        println!("Your number is:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"), //arm
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                print!("you win!");
                break;
            }
        }
    }

   
}
