fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x before in scope is: {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x after in scope is: {}", x);

    let space = "   ";
    let space = space.len() + 1;
    println!("The value of space is: {}", space);
}
