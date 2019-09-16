use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main()
{
    println!("Guess the number");
    let secret_number =rand::thread_rng().gen_range(1,101);
    loop
    {
        println!("Enter your guess");    
    let mut guess =String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");
    let guess: u32=match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,

    };
    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal =>
        {
            println!("Great you won");
            break;
        }
    }

    }
}
































