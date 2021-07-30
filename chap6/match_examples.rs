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
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        // Coin::Penny => 1,
        // 만일 매치 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면
        // 아래와 같이 ...
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
