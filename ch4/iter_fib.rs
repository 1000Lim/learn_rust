struct FibIterator {
    curr: usize,
    next: usize,
}

impl FibIterator {
    fn new() -> Self {
        FibIterator { curr: 1, next: 1 }
    }
}

impl Iterator for FibIterator {
    type Item = usize; // needs to define the types.
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.next;
        self.next = self.curr + self.next;
        self.curr = tmp;
        Some(self.curr)
    }
}

fn main() {
    let fib_iter = FibIterator::new();
    for (i, value) in fib_iter.enumerate() {
        if i >= 10 {
            break;
        }
        print!("{},", value);
    }
    println!();
    let fib_iter = FibIterator::new(); // shadowing the iterator object.
    fib_iter.take(10).for_each(|f| print!("{}, ", f)); // take 10 elements from the iterator, and print the value.
    println!();
}
