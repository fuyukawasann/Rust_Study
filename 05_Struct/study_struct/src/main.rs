struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // 불변 인스턴스
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 가변 인스턴스
    let mut user2 = User {
        active: true,
        username: String::from("someusername1234"),
        email: String::from("soemone@example.com"),
        sign_in_count: 2,
    };

    // 변하는지 확인
    println!("user2 email: {}", user2.email);
    user2.email = String::from("anotherexample@example.com");
    println!("user2 email: {}", user2.email);

    // 함수로 구조체 인스턴스 만들기
    let mut user3 = build_user(String::from("myemail@example.com"), String::from("username12345"));
    println!("user3 email: {}", user3.email);

    // 기존 인스턴스로 새 인스턴스 만들 때 구조체 업데이트 문법 사용
    let user4 = User {
        email: String::from("user4email@example.com"),
        ..user1
    };

    println!("user4 username: {}", user4.username);

    // 튜플 구조체
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 유닛 유사 구조체
    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // 축약법으로 표현하면 username, 으로만으로도 가능하다
        email: email, // 축약법으로 표현하면 email, 으로만으로도 가능하다
        sign_in_count: 1,
    }
}