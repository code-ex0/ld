
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new(port: u16) -> Config {
        Config {
            port,
        }
    }
}