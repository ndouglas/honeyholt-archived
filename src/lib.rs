#![allow(unused_macros)]

#[macro_use]
extern crate volmark;
pub use volmark::*;

pub mod r#macro;
pub use r#macro::*;
pub mod size;
pub mod r#trait;
pub use r#trait::*;

#[cfg(test)]
pub mod test {

  use pretty_env_logger::env_logger::builder;
  use std::env::set_var;

  use super::*;

  #[named]
  pub fn init() {
    let _ = builder().is_test(true).try_init();
    set_var("RUST_BACKTRACE", "1");
  }
}
