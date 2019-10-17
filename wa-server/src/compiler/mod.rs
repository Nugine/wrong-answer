mod gcc;
mod javac;
mod rustc;

pub use gcc::Gcc;
pub use javac::Javac;
pub use rustc::Rustc;

use crate::types::Compiler;
use crate::types::Language;

impl Language {
    // TODO: 
    pub fn get_compiler(&self) -> Option<Box<dyn Compiler>> {
        unimplemented!()
    }
}
