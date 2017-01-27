use std::env;

#[derive(Debug, PartialEq)]
/// The `Config` for a connection to Zendesk's API.
pub struct Config {
    /// The Zendesk URL to connect to.
    pub url: String,
    /// The username for the given Zendesk URL.
    pub username: String,
    /// The API token for the given Zendesk username.
    pub token: Option<String>,
    /// The password for the given Zendesk username.
    pub password: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Possible configuration errors.
pub enum ConfigError {
    /// The environment did not have a username set.
    MissingUsername,
    /// The environment did not have a URL set.
    MissingURL,
    /// The environment did not have a token or password set.
    MissingAuth,
}

impl Config {
    /// Constructs a new `Config`.
    pub fn new() -> Result<Config, ConfigError> {
        let url = match Config::get_url() {
            Ok(val) => val,
            _ => return Err(ConfigError::MissingURL),
        };

        let username = match Config::get_username() {
            Ok(val) => val,
            _ => return Err(ConfigError::MissingUsername),
        };

        let token = match Config::get_token() {
            Ok(val) => Some(val),
            _ => None,
        };

        let password = match Config::get_password() {
            Ok(val) => Some(val),
            _ => None,
        };

        match &token {
            &Some(_) => {}
            &None => {
                match &password {
                    &Some(_) => {}
                    &None => return Err(ConfigError::MissingAuth),
                }
            }
        }

        Ok(Config {
            url: url,
            username: username,
            token: token,
            password: password,
        })
    }

    /// Collects API URL from environment.
    fn get_url() -> Result<String, env::VarError> {
        env::var("ZENDESK_API_URL")
    }

    /// Collects API Token from environment.
    fn get_token() -> Result<String, env::VarError> {
        env::var("ZENDESK_API_TOKEN")
    }

    /// Collects API Username from environment.
    fn get_username() -> Result<String, env::VarError> {
        env::var("ZENDESK_API_USERNAME")
    }

    /// Collects API Password from envrionment.
    fn get_password() -> Result<String, env::VarError> {
        env::var("ZENDESK_API_PASSWORD")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn get_environment() -> Config {
        match Config::new() {
            Ok(val) => val,
            _ => {
                Config {
                    url: String::new(),
                    token: None,
                    password: None,
                    username: String::new(),
                }
            }
        }
    }

    fn set_environment(conf: Config) {
        if conf.username == "".to_string() {
            env::remove_var("ZENDESK_API_USERNAME");
        } else {
            env::set_var("ZENDESK_API_USERNAME", conf.username);
        }

        if conf.url == "".to_string() {
            env::remove_var("ZENDESK_API_URL");
        } else {
            env::set_var("ZENDESK_API_URL", conf.url);
        }

        match conf.token {
            Some(val) => env::set_var("ZENDESK_API_TOKEN", val),
            None => env::remove_var("ZENDESK_API_TOKEN"),
        }

        match conf.password {
            Some(val) => env::set_var("ZENDESK_API_PASSWORD", val),
            None => env::remove_var("ZENDESK_API_PASSWORD"),
        }
    }

    #[test]
    fn test_auth_present() {
        let conf = get_environment();

        env::set_var("ZENDESK_API_USERNAME", "username");
        env::set_var("ZENDESK_API_URL", "http://zendesk.com");
        env::remove_var("ZENDESK_API_TOKEN");
        env::remove_var("ZENDESK_API_PASSWORD");

        assert_eq!(Config::new().unwrap_err(), ConfigError::MissingAuth);

        env::set_var("ZENDESK_API_TOKEN", "abc123");
        let test_config = Config {
            url: String::from("http://zendesk.com"),
            username: String::from("username"),
            password: None,
            token: Some(String::from("abc123")),
        };
        assert_eq!(Config::new().unwrap(), test_config);

        set_environment(conf);
    }
}
