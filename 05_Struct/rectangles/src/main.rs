#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    // 파라미터 두 개 받기
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 튜플로 받기
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // 구조체로 받기
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    // 디버깅 출력
    println!("rect1 is {:?}", rect1);

    // 디버깅 출력 - 보기 편하게 바꾼 버전
    println!("rect1 is {:#?}", rect1);

    // dbg 사용
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}