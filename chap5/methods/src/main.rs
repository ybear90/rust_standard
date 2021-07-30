#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

// Rectangle 구조체 상에 area 메소드 정의하기
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // 더 많은 파라미터를 가진 메소드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // 연관함수 : self parameter를 갖지 않는 함수도 impl 내에 정의하는 것이 허용
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    // 더 많은 파라미터를 가진 메소드
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    // 연관함수 사용 예제
    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
