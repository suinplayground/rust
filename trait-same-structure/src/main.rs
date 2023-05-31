fn main() {
    println!("Hello, world!");
    let my_struct = MyStruct {};

    A::a(&my_struct);
    B::a(&my_struct);
}

trait A {
    fn a(&self) -> String;
}

trait B {
    fn a(&self) -> i16;
}

trait AandB: A + B {}
trait X: AandB {}

struct MyStruct {}

impl A for MyStruct {
    fn a(&self) -> String {
        todo!()
    }
}

impl B for MyStruct {
    fn a(&self) -> i16 {
        todo!()
    }
}

impl AandB for MyStruct {}

impl X for MyStruct {}
