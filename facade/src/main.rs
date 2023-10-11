// FACADE DESIGN PATTERN

// Define subsystems
struct Engine;
impl Engine {
    fn start(&self) {
        println!("Engine started.");
    }
}

struct Lights;
impl Lights {
    fn turn_on(&self) {
        println!("Lights on.");
    }
}

// Define facade
struct Car {
    engine: Engine,
    lights: Lights,
}

// Associate subsystems with facade
impl Car {
    fn new() -> Self {
        Car {
            engine: Engine,
            lights: Lights,
        }
    }

    fn start_car(&self) {
        println!("Cranking up...");
        self.engine.start();
        self.lights.turn_on();
        println!("Ready to drive.");
    }
}

fn main() {
    // Use facade to create a new Car.
    let my_car = Car::new();
    // Use facade to run subsystems.
    my_car.start_car();
}
