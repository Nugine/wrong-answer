#![forbid(unsafe_code)]

pub mod comparer;
pub mod compiler;
pub mod config;
pub mod redis;
pub mod sandbox;
pub mod threads;
pub mod types;

pub use config::GLOBAL_CONFIG;

pub fn hello() {
    println!("Hello, world!");
}
