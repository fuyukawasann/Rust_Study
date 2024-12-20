fn main() {

    // if문
    let number = 3;

    // 예시1
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 예시2
    if number != 0 {
        println!("number was something other than zero");
    }

    // else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if문의 결과를 변수에 할당
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // loop
    // 반복문에서 값 반환하기
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // 루프 라벨
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    // 배열
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // 범위
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

}
