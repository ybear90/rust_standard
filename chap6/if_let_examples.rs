#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Quarter,
    // 50개주 쿼터 동전 모으기...
    Quarter(UsState),
}

// 열거형과 열거형의 variant를 패턴으로서 사용하는 match 표현식
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         // Coin::Penny => 1,
//         // 만일 매치 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면
//         // 아래와 같이 ...
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         // Coin::Quarter => 25,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

// let some_u8_value = Some(0u8);
// match some_u8_value { 
//     Some(3) => println!("three"),
//     _ => (), // 추가하기에 너무 많은 보일러 플레이트 코드
// }

// 덜 타이핑하고, 덜 들여쓰기 함
// match가 강제했던 하나도 빠짐없는 검사를 잃게됨
// syntax sugar
if let Some(3) = some_u8_value {
    println!("three");
}

// sol 1
// let mut count = 0;
// match coin {
//     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//     _ => count += 1,
// }

// sol 2
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
