pub struct Config {
    pub mod_key: String,
    pub terminal: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mod_key: "Super".to_string(),
            terminal: "foot".to_string(),
        }
    }
}
