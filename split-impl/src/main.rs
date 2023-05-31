mod f1;

fn main() {
    println!("Hello, world!");
}

// this trait is defined in other crate, so we can't modify it.
trait Interface {
    fn f1(&self);
    fn f2(&self);
    fn f3(&self);
}

struct RealImpl {}

// we want to implement the Interface separately...
impl Interface for RealImpl {
    fn f1(&self) {
        Self::f1(self);
    }

    fn f2(&self) {
        Self::f2(self);
    }

    fn f3(&self) {
        Self::f3(self);
    }
}
// impl1.rs
impl RealImpl {
    fn f1(&self) {}
}
// impl2.rs
impl RealImpl {
    fn f2(&self) {}
}
// impl3.rs
impl RealImpl {
    fn f3(&self) {}
}
