// 러스트는 표준 라이브러리에 표준 입출력 기능을 제공한다.
// 표준 입출력 기능을 사용하기 위해서는 std::io 모듈을 임포트해야 한다.
use std::io;
// 러스트는 모든 프로그램의 스코프로 가져오는 표준 라이브러리에 정의된 아이템 집합을 가지고 있고, 이를 프렐루드(prelude)라고 한다.
// 만약 원하는 타입이 프렐루드에 없다면 `use` 키워드를 명시적으로 활용하여 타입을 가져와야 한다.

use rand::Rng;

use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    // thread_rng()로 현재 스레드에서만 사용하는 난수 생성기를 생성한다.
    // 이후에 gen_range로 범위를 표현한다. 표현식: start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100); 

    println!("The secret number is: {secret_number}");

    // loop는 무한 루프를 제공한다.
    loop {
        println!("Please input your guess.");

        // 러스트에서는 변수를 선언할 때 타입을 명시적으로 지정해야 한다.
        // 러스트는 변수의 타입을 추론할 수 없기 때문이다.
        // 러스트에서 변수를 선언할 때는 `let` 키워드를 사용한다.
        // 변수의 타입은 `String`이며, 이는 문자열을 저장하는 타입이다.
        // `String::new()`는 새로운 빈 문자열을 생성하는 함수이다.
        // 러스트에서는 기본적으로 불변(immutable)이고 이 뜻은 한 번 값을 집어넣으면 그 값을 바꿀 수 없다는 뜻이다.
        // 변수를 가변(mutable)으로 만들기 위해서는 `mut` 키워드를 사용해야 한다.    
        let mut guess = String::new();

        // 표준 입출력 기능을 사용하여 사용자의 입력을 읽어온다.
        io::stdin()
            // read_line 함수는 사용자의 입력을 읽어오는 함수이다.
            // 그렇기 때문에 가변으로 받아야 한다.
            // &는 참조(reference)를 의미한다.
            // 참조 역시도 기본적으로는 불변이기 때문에 가변으로 바꾸어 선언해야 한다.
            // read_line은 인수로 넘긴 문자열에 사용자 입력을 저장할 뿐만 아니라 하나의 `Result` 값을 반환한다.
            // `Result`는 열거형(enum)으로, 성공(Ok)과 실패(Err)를 나타낸다. -> 여기서 가능한 상태의 값을 배리언트 (variant)라고 한다.
            // 여기서는 실패 시 예외 처리를 하기 위해 `expect` 메서드를 사용한다.
            .read_line(&mut guess)
            // Result 인스턴스가 Err일 경우 expect 메서드는 프로그램을 종료하고 에러 메시지를 출력한다.
            // expect를 호출하지 않는다면 컴파일은 되지만 경고가 나타난다.
            .expect("Failed to read line");


        // trim은 문자열에 자동으로 입력되어 있는 `\n`을 제거한다.
        // parse는 문자열을 다른 타입으로 바꿔준다.
        // 이 부분을 수정하면 잘못된 입력을 처리할 수 있다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {}는 자리표시자 (placeholder)이다.
        // {변수명}: 변수의 값을 출력할 때
        // {}, 수식 : 표현식의 결과를 출력할 때
        // 예시. println!("x = {x}, y + 2 = {}", y + 2);
        println!("You guessed: {guess}");


        // 러스트는 아직 표준 라이브러리에서 랜덤을 지원하지는 않지만 이를 사용할 수 있는 `rand 크레이트(crate)`를 제공한다.
        // crate: 러스트 코드 파일들의 모음
        // 우리가 만드는 프로젝트: 실행 가능한 `바이너리 크레이트(binary crate)`
        // 자체적으로 실행할 수 없고 다른 프로그램을 사용하기 위한 용도: `라이브러리 크레이트(library crate)`
        // rand를 Cargo.toml에 추가한다. -> [dependencies] 아래에 rand = "0.8.5" 추가
        // 카고는 버전 명시 표준인 `유의적 버전(Semantic Versioning/ SemVer)`을 따른다.
        // 여기서 지정자 0.8.5는 ^0.8.5의 축약으로 최소 0.8.5 이상이고 0.9.0 미만의 버전을 사용한다는 뜻이다.
        // 외부 의존성을 추가하면 Crates.io로부터 데이터의 복사본인 레지스트리(registry)에서 해당 의존성이 필요로 하는 모든 것의 최신 버전을 가져온다.

        // 러스트는 기본적으로 `Cargo.lock` 파일을 생성한다.
        // 이 파일은 의존성을 관리하는 데 사용되는 파일로, 의존성의 정확한 버전을 추적한다.
        // 이 파일은 러스트가 의존성을 관리하는 방법을 정확하게 기록하고, 다른 사람이 프로젝트를 빌드할 때 동일한 의존성을 사용하도록 보장한다.

        // 크레이트를 새로운 버전으로 업데이트하려면 `cargo update` 명령어를 사용한다.
        // 지금 경우에는 0.8.5 > 0.8.6으로 업데이트가 되고 0.9.x는 업데이트되지 않는다.
        // 만약 0.9.x로 업데이트하고 싶다면 Cargo.toml에서 0.8.5를 0.9.0으로 변경한다.


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
