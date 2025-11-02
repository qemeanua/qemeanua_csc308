#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self)-> u32{
        self.width * self.height
    }

    fn square(size : u32) -> Rectangle{
        Rectangle{
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1: Rectangle = Rectangle{
        width: 30,
        height: 50,
    };

    println!("rect1: {:?}", rect1);
    println!("Area of rect1: {}", rect1.area());

    let square: Rectangle = Rectangle::square(25);
    println!("Square: {:?}", square);
}
