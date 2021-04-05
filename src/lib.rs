// Copyright (c) 2020 Brandon Thomas <bt@brand.io>

//! Very simple helper functions for environment variables and environment variable-driven
//! `env_logger` configuration.

#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![deny(unused_extern_crates)]
#![deny(unused_imports)]
#![deny(unused_qualifications)]
#![deny(unused_qualifications)]

use env_logger;
use log::{warn, error};
use std::error::Error;
use std::fmt::{Display, Debug, Formatter};
use std::str::FromStr;
use std::{env, fmt};

mod internal;

use internal::get_env_bool_internal;

/// Name of the environment variable Rust's env logger uses
pub const ENV_RUST_LOG : &'static str = "RUST_LOG";

const DEFAULT_LOG_LEVEL: &'static str = "info";

/// Errors with env variables.
#[derive(Debug)]
pub enum EnvError {
  /// Problem parsing the env variable as the desired type.
  ParseError {
    /// Explanation of the parsing failure.
    reason: String
  },
  /// The required environment variable wasn't present.
  RequiredNotPresent,
}

impl Display for EnvError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "EnvError::ParseError")
  }
}

impl Error for EnvError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

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

/// Initialize Rust's env logger.
///
/// The Rust logger reads the desired log level from the `RUST_LOG` environment variable. If this
/// isn't set, the provided default is used. If a default fallback isn't provided to this function,
/// we fall back to `"info"`.
///
/// A more robust logging config might configure on a per-component basis, eg.
/// `"tokio_reactor=warn,hyper=info,debug"`. You can read more in the `log` and `env_logger` crate
/// docs.
pub fn init_env_logger(default_if_absent: Option<&str>) {
  if env::var(ENV_RUST_LOG)
    .as_ref()
    .ok()
    .is_none()
  {
    let default_log_level = default_if_absent.unwrap_or(DEFAULT_LOG_LEVEL);
    println!("Setting default logging level to \"{}\", override with env var {}.",
             default_log_level, ENV_RUST_LOG);
    std::env::set_var(ENV_RUST_LOG, default_log_level);
  }

  env_logger::init();
}
