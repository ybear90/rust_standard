fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!();

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2"); 
    }

    println!();

    println!("---let-if---");

    let condition = true;
    let num = if condition {
        5
    } else {
        6 // "six"는 쓸 수 없다 (변수는 오직 하나의 타입만)
    }; // 일종의 삼항 연산자 개념 ?

    println!("The value of num is: {}", num);

    println!();
    println!("-----while-----");

    let mut numb = 3;

    while numb != 0 {
        println!("{}!", numb);

        numb = numb - 1;
    }

    println!("LIFTOFF!!!");
    
    println!();
    println!("-----for-collections-----");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!();
    println!("-----for-collections2-----");

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
