extern crate rand;

use std::io;
use rand::Rng;

fn main()
{
    let secretnumber = rand::thread_rng().gen_range(1,101);

    println!("hello!!! {}",secretnumber);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Bad");
    println!("you word:{}",guess);


}