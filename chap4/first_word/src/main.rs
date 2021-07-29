fn main() {
    // let mut s = String::from("hello world!");

    // let word = first_word(&s); // word는 5를 갖게 될 것입니다.

    // s.clear(); // 이 코드는 String을 비워서 ""로 만들게 됩니다.

    // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 잇는 스트링은 없음.
    // word는 이제 완전 유효하지 않습니다!
    
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작.
    let word = first_word_rev2(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작.
    let word = first_word_rev2(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word_rev2(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // byte배열 변환

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    s.len()

}

fn first_word_rev(s: &String) -> &str { // &str은 불변참조자
    let bytes = s.as_bytes();

    // String literal은 슬라이스
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_rev2(s: &str) -> &str { // &str은 불변참조자
    let bytes = s.as_bytes();

    // String literal은 슬라이스
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
