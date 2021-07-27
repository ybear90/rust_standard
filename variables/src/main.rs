fn main() {
//let mut x = 5;
//    println!("The value of x is: {}", x);
//    x = 6;
//    println!("The value of x is: {}", x);
    
    // shadowing examples
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x); // The value of x is: 12

    // 첫번째 spaces : string, 두번째 spaces : usize
    let spaces = "   ";
    let spaces = spaces.len();

    // 해당 구문은 에러가 난다
    //let mut spaces2 = "    ";
    //spaces2 = spaces2.len();
}
