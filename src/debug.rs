#[macro_export]
macro_rules! debug {
  ($fmt_str:literal $(, $args:expr)*) => {
    #[cfg(feature = "debug")]
    eprintln!($fmt_str, $(,$args)*);
  }
}
