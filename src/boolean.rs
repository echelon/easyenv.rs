use std::env;
use std::env::VarError;
use log::warn;
use crate::EnvError;

/// Get an environment variable as a bool.
/// If not present or there is an error in parsing, return `None`.
pub fn get_env_bool_optional(env_name: &str) -> Option<bool> {
  match env::var(env_name).as_ref().ok() {
    None => {
      warn!("Env var '{}' not supplied.", env_name);
      None
    },
    Some(val) => match val.as_ref() {
      "TRUE" => Some(true),
      "true" => Some(true),
      "FALSE" => Some(false),
      "false" => Some(false),
      _ => {
        warn!("Env var '{}': error parsing boolean value: {:?}", env_name, val);
        None
      },
    }
  }
}

/// Get an environment variable as a bool, or fall back to the provided default.
/// Returns the default in the event of a parse error.
pub fn get_env_bool_or_default(env_name: &str, default: bool) -> bool {
  get_env_bool_internal(env_name)
      .map(|maybe| match maybe {
        None => {
          warn!("Env var '{}' not supplied. Using default '{}'.", env_name, default);
          default
        },
        Some(val) => val,
      })
      .unwrap_or_else(|e| {
        warn!("Env var '{}': error parsing boolean value: {:?}. Using default '{}'.",
            env_name, e, default);
        default
      })
}

/// Get an environment variable as a bool.
/// If not provided or cannot parse, return an error.
pub fn get_env_bool_required(env_name: &str) -> Result<bool, EnvError> {
  get_env_bool_internal(env_name)
      .and_then(|maybe| match maybe {
        None => {
          warn!("Env var '{}' not supplied.", env_name);
          Err(EnvError::RequiredNotPresent)
        },
        Some(val) => Ok(val),
      })
}

fn get_env_bool_internal(env_name: &str) -> Result<Option<bool>, EnvError> {
  match env::var(env_name).as_deref() {
    Err(err) => match err {
      // TODO: EnvError enum variant for equals sign in env var name
      VarError::NotPresent => Ok(None),
      VarError::NotUnicode(_) => Err(EnvError::NotUnicode),
    }
    Ok(val) => match val {
      "TRUE" => Ok(Some(true)),
      "true" => Ok(Some(true)),
      "FALSE" => Ok(Some(false)),
      "false" => Ok(Some(false)),
      _ => {
        Err(EnvError::ParseError { reason: format!("Couldn't parse as bool: '{}'", val) })
      },
    }
  }
}
