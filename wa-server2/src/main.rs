mod config;
mod lang;
mod redis;
mod sandbox;
mod threads;
mod types;
mod utils;

pub use config::GLOBAL_CONFIG;

#[macro_export]
macro_rules! handle {
    ($ret:expr,$fmt:expr) => {{
        match $ret {
            Err(e) => {
                log::error!($fmt, e);
                panic!(e)
            }
            Ok(r) => r,
        }
    }};
    (@custom $ret:expr,$fmt:expr,$($other:expr,)*) => {{
        match $ret {
            Err(e) => {
                log::error!($fmt, $($other)*);
                panic!(e)
            }
            Ok(r) => r,
        }
    }};
}

fn main() {
    println!("Hello, world!");
}
