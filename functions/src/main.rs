//fn main() {
//    println!("Hello, world!");
//    
//    another_function();
//}
//
//fn another_function(){
//    println!("Another function.");
//}
fn main() {
    another_function(5);

    println!();
    println!("-----another_function2-----");
    another_function2(6, 7);
    
    println!();
    println!("-----표현식 예제-----");

    let x = 5;
    
    let y = {
        let x = 3;
        x + 1 // semi-colon not used.
        // 만약 세미콜론을 표현식 마지막에 추가하면 구문으로 변경 반환값이 아니게됨
    };

    println!("The value of y is: {}", y);
    
    println!();
    println!("-----반환값을 갖는 함수-----");
    
    let z = five();

    println!("The value of z is {}", z);

    println!();
    println!("-----반환값을 갖는 함수2-----");

    let w = plus_one(5);
    println!("The value of w is {}", w);

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
