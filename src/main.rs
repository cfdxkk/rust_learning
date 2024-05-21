
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /**
     * 计算面积
     */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /**
     * 判断能否容下另一个矩形
     */
    fn can_hold (&self, other_rectangle: &Rectangle) -> bool {
        (self.width >= other_rectangle.width && self.height >= other_rectangle.height)
        || (self.width >= other_rectangle.height && self.height >= other_rectangle.width)
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 20,
        height: 10,
    };

    let rectangle2 = Rectangle {
        width: 9,
        height: 150,
    };

    let size = rectangle1.area();
    println!("size is: {}", size);

    let can_hold = rectangle1.can_hold(&rectangle2);
    println!("rectangle1 can hold rectangle2 is {}", can_hold);
    
}
