use lazy_static::lazy_static;
use std::sync::Mutex;

struct Configuration {
    database_url: String,
    api_key: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Configuration> = Mutex::new(Configuration {
        database_url: String::from("http://localhost:5678"),
        api_key: String::from("an_api_key")
    });
}

impl Configuration {
    pub fn get_instance() -> &'static Mutex<Configuration> {
        &CONFIG
    }
}

fn main() {
    let config = Configuration::get_instance();
    let config = config.lock().unwrap();
    println!("Database URL: {}", config.database_url);
    println!("API key: {}", config.api_key);
}
