use my_macro::HelloWorld;

// Define a macro to say hello
macro_rules! say_hello {
    () => {
        println!("Hello, world!")
    };
}

macro_rules! print_fn_name {
    ($func_name:ident) => {
        println!("Function name: {}", stringify!($func_name));
    };
}

fn sample_func() {}

// Define the HelloWorld trait that the derive macro implements
trait HelloWorld {
    fn hello_world();
}

// Use the derive macro
#[derive(HelloWorld)]
struct MyStruct;

fn main() {
    say_hello!();
    print_fn_name!(sample_func);
    MyStruct::hello_world();
}
