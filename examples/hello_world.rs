#![allow(unused_imports)]
#![allow(unused_macros)]

use volmark::*;

#[named]
fn main() {
  init_pretty_env_logger();
  info!("such information");
  warn!("o_O");
  error!("much error");
  trace_enter!();
  println!("{}", env!("RUST_LOG"));
  if cfg!(debug_assertions) {
    println!("Debugging enabled.");
  } else {
    println!("Debugging disabled.");
  }
  println!("Hello, world!");
  trace_exit!();
}
