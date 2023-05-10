// Config module

pub struct Config {

}

impl Config {
    pub fn get(item : &str) -> Option<String> {
        
        match item {
            _ => Config::get_default_config(item),
        }
    }

    fn get_default_config(item : &str) -> Option<String> {
        match item {
            "QDRANT_HOST" => Some("http://10.122.13.226:6333".to_string()),
            "QDRANT_COLLECTION" => Some("e1search".to_string()),
            _ => None,
        }
    }
}