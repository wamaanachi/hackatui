// module for storing user's login credentials and maybe other config related stuff,
//
// Example Folder:
// .local/share/wamaanachi/hackatui/{config files here}
//

use core::fmt;
use dirs;
use std::fs;

#[derive(Debug)]
pub enum ConfigError {
    NoHomeDir,
    NoKey,
    Io(std::io::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::Io(value)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NoHomeDir => write!(f, "could not find home directory"),
            ConfigError::NoKey => {
                write!(f, "no api_key found in .wakatime.cfg or HACKATIME_API_KEY")
            }
            ConfigError::Io(e) => write!(f, "could not read config file: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn load_key() -> Result<String, ConfigError> {
    load_key_with(|name| std::env::var(name).ok())
}

fn load_key_with(f: impl Fn(&str) -> Option<String>) -> Result<String, ConfigError> {
    if let Some(key) = f("HACKATIME_API_KEY") {
        return Ok(key);
    }
    read_cfg()
}

fn read_cfg() -> Result<String, ConfigError> {
    let home = dirs::home_dir().ok_or(ConfigError::NoHomeDir)?;
    let contents = fs::read_to_string(home.join(".wakatime.cfg"))?;
    parse_cfg_file(&contents).ok_or(ConfigError::NoKey)
}

fn parse_cfg_file(contents: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        let (key, value) = line.trim().split_once("=")?;
        if key == "api_key" {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn env_var_works() {
        let key = load_key_with(|name| {
            assert_eq!(name, "HACKATIME_API_KEY");
            Some("from-env".to_string())
        })
        .unwrap();

        assert_eq!(key, "from-env");
    }

    #[test]
    fn read_cfg_works() {
        let contents = "\napi_key=abc123\n";
        assert_eq!(parse_cfg_file(contents), Some("abc123".to_string()));
    }
}
