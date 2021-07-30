// Rust는 우리가 다루지 않은 모든 가능한 경우를 알고있음
// 우리가 어떤 패턴을 잊어먹었는지도 알고 있음!
// match는 하나도 빠뜨리지 않음(exhaustive)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) -> Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // 그전에 명시하지 않은 모든 가능한 경우에 대해 매칭
}

