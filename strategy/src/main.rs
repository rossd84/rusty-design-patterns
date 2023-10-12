// Add trait for payment strategy
trait PaymentStrategy {
    fn pay(&self, amount: f32);
}

// Define types of payments
struct CreditCardPayment;
struct PayPalPayment;

// Implement strategies
impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f32) {
        println!("Credit Card: ${:.2}", amount);
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f32) {
        println!("PayPal: ${:.2}", amount);
    }
}

// Define Shopping cart
struct ShoppingCart {
    payment_strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    // Create new cart and set payment strategy
    fn new(payment_strategy: Box<dyn PaymentStrategy>) -> ShoppingCart {
        ShoppingCart { payment_strategy }
    }

    fn checkout(&self, amount: f32) {
        self.payment_strategy.pay(amount);
    }
}
fn main() {
    // Create payments
    let credit_card = CreditCardPayment;
    let paypal = PayPalPayment;

    // Cart using a credit card
    let user1_cart = ShoppingCart::new(Box::new(credit_card));
    user1_cart.checkout(52.25);

    // Cart using PayPal
    let user2_cart = ShoppingCart::new(Box::new(paypal));
    user2_cart.checkout(3.14);
}
