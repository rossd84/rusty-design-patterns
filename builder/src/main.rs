// Define Product
struct Product {
    name: String,
    price: f32,
    weight: f32,
}

// Define Builder
struct ProductBuilder {
    name: String,
    price: f32,
    weight: f32,
}

impl ProductBuilder {
    // Create new product with default values
    fn new(name: &str, price: f32) -> ProductBuilder {
        ProductBuilder {
            name: name.to_string(),
            price,
            weight: 0.0,
        }
    }

    // Set weight
    fn weight(mut self, weight: f32) -> ProductBuilder {
        self.weight = weight;
        self
    }

    // Creates final Product instance using Builder values
    fn build(&self) -> Product {
        Product {
            name: self.name.clone(),
            price: self.price,
            weight: self.weight,
        }
    }
}
fn main() {
    // Create Product using ProductBuilder
    let product = ProductBuilder::new("Product 1", 9.99).weight(5.2).build();

    println!("Product: {}", product.name);
    println!("Price: ${:.2}", product.price);
    println!("Weight: {:.2} lbs", product.weight);
}
