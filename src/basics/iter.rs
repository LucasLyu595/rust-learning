struct FRange {
    val: f64,
    end: f64,
    incr: f64,
}

fn range(x1: f64, x2: f64, incr: f64) -> FRange {
    FRange {
        val: x1,
        end: x2,
        incr,
    }
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}
pub fn run() {
    // similar to the Python3 `range` function
    let mut iters = 0..3;
    assert_eq!(iters.next(), Some(0));
    assert_eq!(iters.next(), Some(1));
    assert_eq!(iters.next(), Some(2));
    assert_eq!(iters.next(), None);

    let arr = [10, 20, 30];
    // more efficient than using index, by eliminating bounds checking
    for i in arr.iter() {
        print!("{} ", i);
    }

    let slice = &arr;
    // slice will be converted implicitly to an iterator (strongly-typed ha)
    for i in slice {
        print!("{} ", i);
    }

    // need to be explicit about the type of the variable
    let sum: i32 = (0..5).sum();
    println!("{}", sum);

    // no problem to create a new variable with the same name
    let sum: i64 = [10, 20, 30].iter().sum();
    println!("{}", sum);

    let iter = [1, 2, 3, 4, 5];
    let slice = &iter;
    print!("windows: ");
    // `windows` returns an iterator over all contiguous windows of length `size`
    for s in slice.windows(2) {
        print!("{:?} ", s);
    }
    println!();

    // `chunks` returns an iterator over `size` elements of the slice at a time
    for s in slice.chunks(2) {
        print!("{:?} ", s);
    }
    println!();

    for x in range(0.0, 1.0, 0.1) {
        print!("{:.1} ", x);
        // formatting here means one decimal after dot
    }
    println!();
    // let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
    // println!("{:?}", v);
}
