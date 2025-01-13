use std::fmt::{self, Debug};

// typles are convinient but not very descriptive
// the directive make the compiler generate a `Debug` implementation
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}
// the values of a struct will be placed next to each other in memory
// although you should not assume any particular memory layout
// since the compiler will organize the memory for efficiency, not size, and there may be padding

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        // takes a reference self argument
        // `self` is used explicitly and is passed as a reference, short for `self: &Person`
        format!("{} {}", self.first_name, self.last_name)
    }

    fn copy(&self) -> Self {
        // `Self` refers to the struct type
        Self::new(&self.first_name, &self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        // methods may allow the data to be modified using a mutable self argument
        self.first_name = name.to_string();
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_tuple(self) -> (String, String) {
        // data will be moved when plain self argument is used
        (self.first_name, self.last_name)
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
        // `write!` is a very useful macro - here `f` is anything that implements `Write`
    }
}

type NodeBox<T> = Option<Box<Node<T>>>;
// no need for forward declaration
// `Box` is a smart pointer; note that no 'unboxing' was needed to call `Node` methods on it

#[allow(dead_code)]
#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: NodeBox<T>,
    right: NodeBox<T>,
}

impl<T: PartialOrd + Debug> Node<T> {
    fn new(s: T) -> Node<T> {
        Node {
            payload: s,
            left: None,
            right: None,
        }
    }

    fn boxer(node: Node<T>) -> NodeBox<T> {
        Some(Box::new(node))
    }

    fn set_left(&mut self, node: Node<T>) {
        self.left = Self::boxer(node);
    }

    fn set_right(&mut self, node: Node<T>) {
        self.right = Self::boxer(node);
    }

    fn insert(&mut self, data: T) {
        if data < self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data)),
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data)),
            }
        }
    }

    fn visit(&self) {
        if let Some(ref n) = self.left {
            n.visit();
        }
        println!("{:?}", self.payload);
        if let Some(ref n) = self.right {
            n.visit();
        }
    }
}

pub fn run() {
    // let p = Person {
    //     first_name: String::from("John"),
    //     last_name: "Doe".to_string(),
    // };
    let mut p = Person::new("John", "Doe");
    p.set_first_name("Jane");
    println!("{:?}", p);
    println!("{}", p);
    let q = p.copy();
    println!("{:?}", q.to_tuple());

    let mut root = Node::new("root".to_string());
    root.insert("one".to_string());
    root.insert("two".to_string());
    root.insert("four".to_string());
    // println!("{:#?}", root); // '#' stands for 'extended'
    root.visit();
}
