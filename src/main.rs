// Define trait Shape
trait Shape {
    fn draw(&self);
}

// Define struct Circle
struct Circle;

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

// Define struct Rectangle
struct Rectangle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle");
    }
}

// Define trait ShapeFactory
trait ShapeFactory {
    fn create_shape(&self) -> Box<dyn Shape>;
}

// Implement ShapeFactory for Circle
struct CircleFactory;

impl ShapeFactory for CircleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Circle)
    }
}

// Implement ShapeFactory for Rectangle
struct RectangleFactory;

impl ShapeFactory for RectangleFactory {
    fn create_shape(&self) -> Box<dyn Shape> {
        Box::new(Rectangle)
    }
}

// Main function
fn main() {
    let circle_factory = CircleFactory;
    let rectangle_factory = RectangleFactory;

    let circle = circle_factory.create_shape();
    let rectangle = rectangle_factory.create_shape();

    circle.draw();
    rectangle.draw();
}
