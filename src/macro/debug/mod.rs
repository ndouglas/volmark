/// Logs a variable and its value, if at RUST_LOG=trace.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate volmark;
/// # use volmark::*;
/// # #[named]
/// # fn main() {
/// let five = 5;
/// trace_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! trace_var {
  ($var: expr) => {{
    #[cfg(debug_assertions)]
    {
      use log::*;
      trace!("{} = {:?}", stringify!($var), $var);
    }
  }};
}

/// Logs a variable and its value, if at RUST_LOG>=debug
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate volmark;
/// # use volmark::*;
/// # #[named]
/// # fn main() {
/// let five = 5;
/// debug_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! debug_var {
  ($var: expr) => {{
    #[cfg(debug_assertions)]
    {
      use log::*;
      debug!("{} = {:?}", stringify!($var), $var);
    }
  }};
}

/// Logs a variable and its value, if at RUST_LOG>=info
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate volmark;
/// # use volmark::*;
/// # #[named]
/// # fn main() {
/// let five = 5;
/// info_var!(five);
/// # }
/// ```
#[macro_export]
macro_rules! info_var {
  ($var: expr) => {{
    #[cfg(debug_assertions)]
    {
      use log::*;
      info!("{} = {:?}", stringify!($var), $var);
    }
  }};
}

/// Traces entry into a function, if at RUST_LOG=trace
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate volmark;
/// # use volmark::*;
/// # #[named]
/// # fn main() {
/// trace_enter!();
/// # }
/// ```
#[macro_export]
macro_rules! trace_enter {
  () => {{
    #[cfg(debug_assertions)]
    {
      use log::*;
      trace!("[ENTER] {} @ line {}", function_name!(), line!());
    }
  }};
}

/// Traces exit from a function, if at RUST_LOG=trace
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate volmark;
/// # use volmark::*;
/// # #[named]
/// # fn main() {
/// trace_exit!();
/// # }
/// ```
#[macro_export]
macro_rules! trace_exit {
  () => {{
    #[cfg(debug_assertions)]
    {
      use log::*;
      trace!("[EXIT] {} @ line {}", function_name!(), line!());
    }
  }};
}
