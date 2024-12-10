fn main() {
    
    // 예제 1 - 문자열 리터럴
    let s = "hello";
    println!("{s}");

    // from 사용한 String 타입
    let s = String::from("hello");
    println!("{s}");

    // 문자열 수정 가능하게 변경
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가합니다

    println!("{}", s); // 이 줄이 `hello, world!`를 출력합니다

    // 예제 2 - 바인딩
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // String 타입
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, {s2}"); // 에러 발생
    println!("{}, world!", s2);


    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // 예제 3 - 함수 호출
    let s = String::from("hello");

    takes_ownership(s); // s의 값이 함수 안으로 이동했습니다...

    let x = 5;

    makes_copy(x);

    // 예제 4 - 반환 값과 스코프
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    // s2는 복사되었기 때문에 유효하지 않다.
    println!("s1 = {s1}, s3 = {s3}");

    // 예제 5
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String { 
    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다
    some_string
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String {
    a_string 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}