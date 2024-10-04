// Define the trait
trait MyTrait {
    fn my_function(&self);
}

// Import the procedural macro
use my_macro::MyTrait;

// Define a struct and derive the procedural macro
#[derive(MyTrait)]
struct MyStruct;

fn main() {
    let s = MyStruct;
    s.my_function();
}
