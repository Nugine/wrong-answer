mod lang;
mod sandbox;
mod types;
mod utils;
mod threads;
mod redis;

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
