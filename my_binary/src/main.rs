// Define the trait
trait MyTrait {
    fn my_function(&self);
}

// Import the procedural macro
use my_macro::MyTrait;
use route_macro::route;

#[route("/home")]
fn home() {
    println!("This is the home page");
}
// Define a struct and derive the procedural macro
#[derive(MyTrait)]
struct MyStruct;

fn main() {
    let s = MyStruct;
    s.my_function();
    home();
}
