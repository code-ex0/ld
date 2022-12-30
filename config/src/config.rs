
pub struct Config {
    pub port: u16,
    pub url: String,
}

impl Config {
    pub fn new(port: u16, url: String) -> Config {
        Config {
            port,
            url,
        }
    }
}