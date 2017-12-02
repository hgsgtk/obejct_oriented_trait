trait SayHello : std::fmt::Display {
    fn say_hello(&self) {
        println!("Hello. This is {}.", self);
    }
}

trait SayGoodbye : std::fmt::Display {
    fn say_goodbye(&self) {
        println!("Goodbye. This is {}.", self);
    }
}

impl SayHello for i32 {}
impl SayHello for str {
    fn say_hello(&self) {
        println!("Hi. I'm {}.", self);
    }
}

fn main() {
    42.say_hello();
    "Alice".say_hello();
}

/* 書き方1
fn greeting<T: SayHello + SayGoodbye>(x: T) {
    x.say_hello();
    x.say_goodbye();
}
*/

/* 書き方2
fn greeting<T><x: T) where T: SayHello + SayGoodbye {
    x.say_hello();
    s.say_goodbye();
}
*/

/* 書き方3
fn greeting<T>(x: T) where T: SayHello + SayGoodbye {
    <T as SayHello>::say_hello(&x);
    <T as SayGoodbye>::say_goodbye(&x);
}
*/

/* 書き方4
impl<'a, T: ?Sized + SayHello> SayHello for &'a T {
    fn say_hello(&self) {
        <T as SayHello>::say_hello(self);
    }
}*/

// Box::new(42) as BOX<SayHello>

