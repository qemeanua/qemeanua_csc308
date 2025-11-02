#[derive (Debug)]

struct Circle{
    radius : f32,
}

impl Circle{
    fn area(&self) -> f32{
        3.142 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32{
        2.0 * 3.142 * self.radius
    }
}

fn main() {
    let circ1 : Circle = Circle{
        radius : 20.0,
    };

    println!("Area of circle: {}", circ1.area());
    println!("Perimeter of circle: {}", circ1.perimeter());
}
