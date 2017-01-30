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
    pub fn new(url: String,
               username: String,
               password: Option<String>,
               token: Option<String>)
               -> Result<Config, ConfigError> {

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
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_auth_present() {
        assert_eq!(Config::new(String::from("username"),
                               String::from("http://zendesk.com"),
                               None,
                               None)
                       .unwrap_err(),
                   ConfigError::MissingAuth);

        let test_config = Config {
            url: String::from("http://zendesk.com"),
            username: String::from("username"),
            password: None,
            token: Some(String::from("abc123")),
        };
        assert_eq!(Config::new(String::from("http://zendesk.com"),
                               String::from("username"),
                               None,
                               Some(String::from("abc123")))
                       .unwrap(),
                   test_config);
    }
}
