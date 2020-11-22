#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }

    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn main() {
    let rectangle1 = Rectangle::new(3, 5);
    println!("{}", rectangle1.area());
}
