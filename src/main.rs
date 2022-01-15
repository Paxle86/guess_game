use std::io;
use rand::Rng;

fn main() {

    // tao ra 1 so random trong moi lan chay truong trinh
    // nguoi dung nhap so vao cml de doan cai bi mat ma chuwong trinh tao ra


    let game_name = "Guess the number !!!";
    println!("{}", game_name);
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is: {}", secret_number);

    let mut flag = false;
    while flag == false {
        println!("Please input your guess.");
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read liner");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        // if guess > secret_number {
        //     println!("Too big!")
        // } else if guess < secret_number {
        //     println!("Too small");
        // } else {
        //     flag = true;
        //     println!("You win !!!!!!!");
        // }
        if guess > secret_number || guess < secret_number {
            println!("Wrong number");
        } else {
            flag = true;
            println!("You win !!!!!!!");
        }
    }
}
