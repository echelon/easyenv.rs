use std::env;
use std::fmt::{Debug, Display};
use std::str::FromStr;
use log::{error, warn};
use crate::EnvError;

/// Get an environment variable as a number, or fall back to the provided default if not set.
/// If the env var is present but can't be parsed, an error is returned instead.
pub fn get_env_num<T>(env_name: &str, default: T) -> Result<T, EnvError>
  where T: FromStr + Display,
        T::Err: Debug
{
  match env::var(env_name).as_ref().ok() {
    None => {
      warn!("Env var '{}' not supplied. Using default '{}'.", env_name, default);
      Ok(default)
    },
    Some(val) => {
      val.parse::<T>()
          .map_err(|e| {
            error!("Can't parse value '{:?}'. Error: {:?}", val, e);
            EnvError::ParseError { reason: format!("Can't parse value: {:?}", e) }
          })
    },
  }
}
