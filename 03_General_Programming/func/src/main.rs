fn five() -> i32 {
    5
}

fn main() {
    another_function(5, 'h');


    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");


    // 반환 값을 가지는 함수
    // 예제 1
    let x = five();
    println!("The value of x is: {x}");

    // 예제 2
    let x = plus_one(5);
    println!("The value of x is: {x}");

}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
