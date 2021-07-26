extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        // :: <- new가 String 타입의 연관함수 즉, String 타입을 위한 함수 
        let mut guess = String::new(); // mut <- 가변 변수 만드는 keyword
        
        // & <- 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법 제공을 위한 참조자
        io::stdin().read_line(&mut guess)         
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            // 숫자를 입력하면 할당, 아니면 pass
            Ok(num) => num,
            Err(_) => continue,
        };
            
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
