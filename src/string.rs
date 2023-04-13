use crate::EnvError;
use log::warn;
use std::env;

/// Get an environment variable as an optional `String`.
pub fn get_env_string_optional(env_name: &str) -> Option<String> {
  match env::var(env_name).as_ref().ok() {
    Some(s) => Some(s.to_string()),
    None => {
      warn!("Env var '{}' not supplied.", env_name);
      None
    },
  }
}

/// Get an environment variable as a `String`, or fall back to the provided default.
pub fn get_env_string_or_default(env_name: &str, default: &str) -> String {
  match env::var(env_name).as_ref().ok() {
    Some(s) => s.to_string(),
    None => {
      warn!("Env var '{}' not supplied. Using default '{}'.", env_name, default);
      default.to_string()
    },
  }
}

/// Get an environment variable as a `String`, or return an error.
pub fn get_env_string_required(env_name: &str) -> Result<String, EnvError> {
  match env::var(env_name).as_ref().ok() {
    Some(s) => Ok(s.to_string()),
    None => {
      warn!("Required env var '{}' not supplied.", env_name);
      Err(EnvError::RequiredNotPresent)
    },
  }
}

