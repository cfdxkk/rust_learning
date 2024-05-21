
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 20,
        height: 10,
    };

    let size = rectangle.area();
    println!("size is: {}", size);
}
