// enums are types with have a few definite values
#[derive(Debug, PartialEq)]
// `PartialEq` required for comparison
// `Debug` required for `assert_eq!` to format the output
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &str {
        match *self {
            // can also work without `*`
            // TODO:check reason, possible because of
            // Directiona and &Direction are comparable?
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}
// enum can implement traits
// but `as_str` is more efficient because it avoids heap allocation
// whereas `#[derive(Debug)]` is more convenient but involves creating a new `String`

// C style enums
// initialized with an integer value, can be converted into that integer with a type case
#[allow(dead_code)]
#[derive(PartialEq, PartialOrd)]
// enums do have a natural ordering, but you have to ask nicely
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50,
}

// only need to give the first variant a value, and thereafter the value goes up by one each time
#[allow(dead_code)]
enum Difficulty {
    Easy = 1,
    Medium,
    Hard,
}

pub fn run() {
    let start = Direction::Up;
    let mut d = start;
    for _ in 0..8 {
        println!("Direction: {}", d.as_str());
        d = d.next();
    }
    assert_eq!(d, Direction::Up);

    let s = Speed::Slow;
    let speed = s as u32;
    println!("Speed: {}", speed);
}
