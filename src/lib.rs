#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications,
        warnings)]
//! # zendesk
//!
//! The `zendesk` crate provides an interface with Zendesk's API.

pub use self::config::{Config, ConfigError};

mod client;
mod config;
