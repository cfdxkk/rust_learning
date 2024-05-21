
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 20,
        height: 10,
    };

    let size = area(&rectangle);
    println!("size is: {}", size);
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
