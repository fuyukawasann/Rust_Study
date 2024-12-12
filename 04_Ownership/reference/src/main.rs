fn main() {

    // Example 1
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    // Example 2
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // Example 3
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // Example 4
    let no_dangle_string = no_dangle();
    println!("{}", no_dangle_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}