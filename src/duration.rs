use crate::EnvError;
use log::warn;
use std::env;
use std::time::Duration;

/// Get an environment variable as a `Duration` in seconds.
/// If not provided or cannot parse, return an error.
pub fn get_env_duration_seconds_required(env_name: &str) -> Result<Duration, EnvError> {
  get_env_duration_seconds_internal(env_name)
    .and_then(|maybe| match maybe {
      None => {
        warn!("Env var '{}' not supplied.", env_name);
        Err(EnvError::RequiredNotPresent)
      },
      Some(val) => Ok(val),
    })
}

/// Get an environment variable as a `Duration` in seconds.
/// If not present or there is an error in parsing, return `None`.
pub fn get_env_duration_seconds_optional(env_name: &str) -> Option<Duration> {
  match get_env_duration_seconds_internal(env_name) {
    Err(e) => {
      warn!("Env var '{}': error parsing numeric value: `{:?}`. Returning no value.", env_name, e);
      None
    },
    Ok(None) => {
      warn!("Env var '{}' not present.", env_name);
      None
    },
    Ok(Some(value)) => Some(value),
  }
}

/// Get an environment variable as a `Duration` in seconds, or fall back to the provided default.
/// Returns the default in the event of a parse error.
pub fn get_env_duration_seconds_or_default(env_name: &str, default: Duration) -> Duration {
  get_env_duration_seconds_internal(env_name)
    .map(|maybe| match maybe {
      None => {
        warn!("Env var '{}' not supplied. Using default '{:?}'.", env_name, default);
        default
      },
      Some(val) => val,
    })
    .unwrap_or_else(|e| {
      warn!("Env var '{}': error parsing numeric value: {:?}. Using default '{:?}'.",
            env_name, e, default);
      default
    })
}

fn get_env_duration_seconds_internal(env_name: &str) -> Result<Option<Duration>, EnvError> {
  match env::var(env_name).as_ref().ok() {
    None => {
      Ok(None)
    },
    Some(val) => match val.parse::<u64>() {
      Ok(number) => Ok(Some(Duration::from_secs(number))),
      Err(_) => Err(EnvError::ParseError { reason: format!("Couldn't parse as number: '{}'", val) })
    }
  }
}
