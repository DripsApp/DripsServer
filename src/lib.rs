pub mod ugc;
pub mod comments;
pub mod decoder;
pub mod config;
pub mod bean;
pub mod response;
pub mod verify;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
#[macro_use]
extern crate dotenv_codegen;
rust_i18n::i18n!("locales");