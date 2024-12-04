fn main() {
    // 부동 소수점의 용례
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x: {x}, y: {y}");


    // 수학적 연산
    // 덧셈
    let sum = 5 + 10;
    println!("sum: {sum}");

    // 뺄셈
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");

    // 곱셈
    let product = 4 * 30;
    println!("product: {product}");

    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결괏값은 -1입니다
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");

    // 나머지 연산
    let remainder = 43 % 5;
    println!("remainder: {remainder}");


    // 불리언 타입
    let t = true;
    println!("t: {t}");

    let f: bool = false; // 명시적인 타입 어노테이션
    println!("f: {f}");


    // char
    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");


    // tuple
    // Example 1
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    
    println!("x: {x}, y: {y}, z: {z}");

    // Example 2
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");


    // array
    let a = [1, 2, 3, 4, 5];
    
    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");

}
