struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// lifetime parameter가 없으면 아래의 구조체는 에러를 발생시킨다
//struct User {
//    username: &str,
//    email: &str,
//    sign_in_count: u64,
//    active: bool,
//}

// 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        //active: user1.active,
        //sign_in_count: user1.sign_in_count,
        ..user1 // 나머지 필드는 user1에서 재사용
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 사용자의 이메일과 이름을 받아 User 구조체의 인스턴스를 반환하는 build_user 함수
fn build_user(email: String, username: String) -> User {
    User {
        //email: email,
        //username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
