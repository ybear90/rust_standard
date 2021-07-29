fn main() {
    let reference_to_nothing = no_dangle();
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    // &s <- error 발생 : dangle의 코드가 끝나면 할당해제 되므로
//    // 무효화된 String을 가리키게 되는 위험성이 존재
//}
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
