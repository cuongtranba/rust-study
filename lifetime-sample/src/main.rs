// alway copy because primitive type is cheap and it auto implement copy trail
fn max_without_lifetime(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn max_string<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b { a } else { b }
}

fn main() {
    let a = 10;
    let b = 20;
    let max = max_without_lifetime(a, b);
    println!("The maximum value is {}", max);
    // you can
    println!("a is {}", a);

    let str1 = "Hello";
    let str2 = "World";
    let max_str = max_string(str1, str2);
    println!("The maximum string is {}", max_str);
}
