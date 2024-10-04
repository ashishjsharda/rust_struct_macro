use rust_struct_macro::MyTrait;

// Define the trait in the consumer crate.
pub trait MyTrait {
    fn my_function(&self);
}

#[derive(MyTrait)]
struct MyStruct;

fn main() {
    let instance = MyStruct;
    instance.my_function(); // Outputs: Hello, I am MyStruct!
}
