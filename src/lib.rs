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

use log::{warn, error};
use env_logger;
use std::env;
use std::fmt::{Display, Debug};
use std::str::FromStr;

/// Name of the environment variable Rust's env logger uses
pub const ENV_RUST_LOG : &'static str = "RUST_LOG";

const DEFAULT_LOG_LEVEL: &'static str = "info";

/// Errors with env variables.
pub enum EnvError {
  /// Problem parsing the env variable as the desired type.
  ParseError
}

/// Get an environment variable as a `String`, or fall back to the provided default.
pub fn get_env_string(env_name: &str, default: &str) -> String {
  match env::var(env_name).as_ref().ok() {
    Some(s) => s.to_string(),
    None => {
      warn!("Env var '{}' not supplied. Using default '{}'.", env_name, default);
      default.to_string()
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
          EnvError::ParseError
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
