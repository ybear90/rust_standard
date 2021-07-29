fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2); // s1을 사용하면 에러가 남(move) 

    // 정말로 복사하고자 한다면 이렇게 해야 한다.
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // 정수형과 같이 컴파일 타임에 결정되어 있는 크기의 타입은 스택에 모두 저장
    // 실제 값의 복사본이 빠르게 만들어 질 수 있음.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Copy trait, and Drop trait에 대해선 추후에 알아보는 것으로...
    // Copy가 가능한 몇가지 타입들
    // 1. u32와 같은 모든 정수형 타입
    // 2. true와false값을 갖는 부울린 타입 bool
    // 3. f64와 같은 모든 부동 소수점 타입들
    // 4. Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 x

}
