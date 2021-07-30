// Quit : 연관된 데이터가 전혀 없음
// Move : 익명 구조체 포함
// Write : 하나의 String을 포함
// ChangeColor : 세 개의 i32를 포함
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // 메소드 내용은 여기 정의할 수 있습니다.
    }
}

let m = Message::Write(String::from("hello"));
m.call(); // 이게 실행되면 call 메소드 안에서 self가 됨
