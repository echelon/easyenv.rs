use crate::EnvError;
use log::warn;
use std::env::VarError;
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Get an environment variable as a `PathBuf`.
/// If not provided or cannot parse, return an error.
pub fn get_env_pathbuf_required(env_name: &str) -> Result<PathBuf, EnvError> {
  get_env_pathbuf_internal(env_name)
    .and_then(|maybe| match maybe {
      None => {
        warn!("Env var '{}' not supplied.", env_name);
        Err(EnvError::RequiredNotPresent)
      },
      Some(val) => Ok(val),
    })
}

/// Get an environment variable as a `PathBuf`.
/// If not present or there is an error in parsing, return `None`.
pub fn get_env_pathbuf_optional(env_name: &str) -> Option<PathBuf> {
  match get_env_pathbuf_internal(env_name) {
    Err(e) => {
      warn!("Env var '{}': error parsing PathBuf value: `{:?}`. Returning no value.", env_name, e);
      None
    },
    Ok(None) => {
      warn!("Env var '{}' not present.", env_name);
      None
    },
    Ok(Some(value)) => Some(value),
  }
}

/// Get an environment variable as a `PathBuf`, or fall back to the provided default.
/// Returns the default in the event of a parse error.
pub fn get_env_pathbuf_or_default<P: AsRef<Path>>(env_name: &str, default_value: P) -> PathBuf {
  get_env_pathbuf_internal(env_name)
    .map(|maybe| match maybe {
      None => {
        let default_path = default_value.as_ref().to_path_buf();
        warn!("Env var '{}' not supplied. Using default '{:?}'.", env_name, &default_path);
        default_path
      },
      Some(val) => val,
    })
    .unwrap_or_else(|e| {
      let default_path = default_value.as_ref().to_path_buf();
      warn!("Env var '{}': error: {:?}. Using default '{:?}'.",
            env_name, e, &default_path);
      default_path
    })
}

fn get_env_pathbuf_internal(env_name: &str) -> Result<Option<PathBuf>, EnvError> {
  match env::var(env_name).as_ref() {
    Err(err) => match err {
      // TODO: EnvError enum variant for equals sign in env var name
      VarError::NotPresent => Ok(None),
      VarError::NotUnicode(_) => Err(EnvError::NotUnicode),
    }
    Ok(val) => {
      match PathBuf::from_str(val) {
        Ok(path) => Ok(Some(path)),
        Err(_err) => Err(EnvError::ParseError {
          reason: "error parsing PathBuf from value".to_string()
        }),
      }
    }
  }
}
