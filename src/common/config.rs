use std::env;

pub struct Config {
    pub port: u16,
}

const DEAFULT_PORT: u16 = 8080;

impl Config {
    pub fn load_from_env() -> Config {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(DEAFULT_PORT);
        Config { port }
    }
}
