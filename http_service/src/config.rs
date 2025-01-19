use std::cmp::Ordering;
use std::env;
use std::str::FromStr;
use tracing::Level;

const DEFAULT_PORT: u16 = 3000;

pub struct Config {
    pub environment: Environment,
    pub port: u16,
    pub log_level: Level,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Staging,
    Test,
}

impl Config {
    pub fn env() -> Environment {
        env::var("RUST_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .try_into()
            .unwrap_or(Environment::Development)
    }

    pub fn port() -> u16 {
        env::var("PORT")
            .map(|p| p.parse::<u16>().unwrap_or(DEFAULT_PORT))
            .unwrap_or(DEFAULT_PORT)
    }

    pub fn log_level() -> Level {
        env::var("RUST_LOG")
            .ok()
            .and_then(|l| Level::from_str(&l).ok())
            .unwrap_or(Level::INFO)
    }

    pub fn from_env() -> Self {
        Self {
            environment: Self::env(),
            port: Self::port(),
            log_level: Self::log_level(),
        }
    }

    pub fn setup_logging(&self) {
        let verbose = self.log_level.cmp(&Level::DEBUG) >= Ordering::Equal;

        let builder = tracing_subscriber::fmt()
            .with_target(verbose)
            .with_level(true)
            .with_thread_ids(verbose)
            .with_file(verbose)
            .with_line_number(verbose)
            .with_timer(tracing_subscriber::fmt::time::time())
            .with_max_level(self.log_level);

        if self.environment == Environment::Production {
            builder.json().flatten_event(true).init();
        } else {
            builder.init();
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            "staging" => Ok(Environment::Staging),
            "test" => Ok(Environment::Test),
            other => Err(format!(
                "{} is not a supported environment. Use either 'development', 'test', or 'production'",
                other
            )),
        }
    }
}
