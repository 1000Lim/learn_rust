#[derive(Debug)] // Derive the Debug trait
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::AddAssign + std::ops::SubAssign + Copy, // restrict T to types that implement AddAssign, SubAssign, and Copy
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn add(&mut self, other: &Point<T>) {
        self.x += other.x;
        self.y += other.y;
    }
    fn sub(&mut self, other: &Point<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

fn main() {
    let mut p1 = Point::new(3, 4);
    let p2 = Point::new(1, 2);
    println!("p1: {:?}", p1);
    p1.add(&p2);
    println!("p1 after add: {:?}", p1);
    p1.sub(&p2);
    println!("p1 after sub: {:?}", p1);
}
