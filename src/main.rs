trait SayHello : std::fmt::Display {
    fn say_hello(&self) {
        println!("Hello, This is {}.", self);
    }
}

impl SayHello for i32 {}
impl SayHello for str {
    fn say_hello(&self) {
        println!("Hi. I'm {}.", self);
    }
}
impl<'a, T: ?Sized + SayHello> SayHello for &'a T {
    fn say_hello(&self) {
        <T as SayHello>::say_hello(self);
    }
}

fn main() {
    let v = vec![
        Box::new(42) as Box<SayHello>,
        Box::new("Alice") as Box<SayHello>,
    ];
    for x in &v {
        x.say_hello();
    }
}
