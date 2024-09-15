
trait Shape {
    fn area(&self) -> f64;
}

struct Circle  {
    radius: f64,
}

impl Circle {
    fn get_area_of_circle(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.get_area_of_circle()
    }
}

struct Square {
    side: f64
}

impl Square {
    fn get_area_of_square(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.get_area_of_square()
    }
}

fn print_area(shape: &dyn Shape) {
    println!("The area is: {}", shape.area());
}


fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 5.0 };
    print_area(&circle);
    print_area(&square);
}
