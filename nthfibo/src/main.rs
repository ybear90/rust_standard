use std::io;

fn main() {
    println!("please enter the fibo");

    let mut fibo_num = String::new();

    io::stdin().read_line(&mut fibo_num)
        .expect("Failed to read_line");

    let num: u32 = fibo_num.trim().parse()
        .expect("Please type the number");

    println!("nth fibo is {}", nth_fibo(num));
}

fn nth_fibo(x: u32) -> u32 {
    return if x <= 2 { 1 } else {
        if x == 0 { 
          0 
        } else { 
          nth_fibo(x-1) + nth_fibo(x-2) 
        }
    }
}
