trait Shape {
    fn draw(&self);
}

struct Circle;
impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle.")
    }
}

struct Rectangle;
impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle.")
    }
}

trait ShapeFactory {
    fn create_shape(&self, shape_type: &str) -> Box<dyn Shape>;
}

struct ConcreteShapeFactory;
impl ShapeFactory for ConcreteShapeFactory {
    fn create_shape(&self, shape_type: &str) -> Box<dyn Shape> {
        match shape_type {
            "circle" => Box::new(Circle),
            "rectangle" => Box::new(Rectangle),
            _ => panic!("Invalid shape"),
        }
    }
}

fn main() {
    let factory = ConcreteShapeFactory;

    let circle = factory.create_shape("circle");
    circle.draw();

    let rectangle = factory.create_shape("rectangle");
    rectangle.draw();
}
