mod foo {
    #[derive(Debug)]
    pub struct Foo {
        s: &'static str, // better to hide the insides fo a struct, and only allow access through methods
    }
    // the set of functions and types exported from a module is called its interface
    impl Foo {
        pub fn new(s: &'static str) -> Foo {
            Foo { s }
        }
        pub fn get_s(&self) -> &'static str {
            self.s
        }
    }
}

// explicitly exported
pub fn run() {
    // let f = foo::Foo { s: "hello" }; // error: field `s` of struct `foo::Foo` is private
    let f = foo::Foo::new("hello");
    println!("{}", f.get_s());
}
