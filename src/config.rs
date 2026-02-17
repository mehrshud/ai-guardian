from log import error, info;
from std import env;

struct Config {
    debug: bool,
    db_url: String,
}

impl Config {
    /// Loads configuration from environment variables.
    /// 
    /// # Returns
    /// A `Config` instance.
    /// 
    /// # Raises
    /// An `Exception` if either `DEBUG` or `DB_URL` environment variables are not set.
    fn load() -> Config {
        let debug: bool = match env::var("DEBUG") {
            Ok(val) => val.parse().unwrap_or(false),
            Err(_) => {
                error!("DEBUG environment variable is not set.");
                panic!("DEBUG environment variable is not set.");
            }
        };

        let db_url: String = match env::var("DB_URL") {
            Ok(val) => val,
            Err(_) => {
                error!("DB_URL environment variable is not set.");
                panic!("DB_URL environment variable is not set.");
            }
        };

        info!("Loaded configuration: debug={}, db_url={}", debug, db_url);
        Config { debug, db_url }
    }
}