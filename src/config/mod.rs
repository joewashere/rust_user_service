use config::{Config, File, Environment};

pub fn load_configuration() -> Result<Config, config::ConfigError> {
    let mut settings = Config::default();
    
    // Start off by merging with the "default" configuration file
    settings.merge(File::with_name("config.toml"))?;
    
    // Add in overrides from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_SECRETS__JWT_SECRET_KEY=...` would override the jwt_secret_key
    settings.merge(Environment::with_prefix("app").separator("__"))?;
    
    Ok(settings)
}