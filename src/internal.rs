use crate::EnvError;
use std::env;

pub (crate) fn get_env_bool_internal(env_name: &str) -> Result<Option<bool>, EnvError> {
  match env::var(env_name).as_ref().ok() {
    None => {
      Ok(None)
    },
    Some(val) => match val.as_ref() {
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
