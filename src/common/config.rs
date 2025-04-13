use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: u16,
    pub api_origin: String,
}

const DEAFULT_PORT: u16 = 8080;

impl Config {
    pub fn load_from_env() -> Config {
        dotenv().ok();
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(DEAFULT_PORT);
        let api_origin = env::var("API_ORIGIN").expect("missing 'API_ORIGIN' env");
        Config { port, api_origin }
    }
}
