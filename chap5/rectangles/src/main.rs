// fn main() {
//     // let length1 = 50;
//     // let width1 = 30;
//     // tuple을 이용한 리펙토링
//     let rect1 = (50, 30);
// 
//     println!(
//         "The area of the rectangle is {} square pixels.",
//          area(rect1) // area(length1, width1)    
//     );
// }

//fn area(length: u32, width: u32) -> u32 {
//    length * width
//}
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 구조체를 이용한 리팩터링: 의미를 더 추가하기
// struct Rectangle {
//     length: u32,
//     width: u32,
// }
// 
// fn main() {
//     let rect1 = Rectangle { length: 50, width: 30 };
// 
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }
// 
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.width
// }
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
    println!();
    println!("rect1 is {:#?}", rect1);
}
