mod gcc;
mod javac;
mod rustc;

pub use gcc::Gcc;
pub use javac::Javac;
pub use rustc::Rustc;

use crate::types::Compiler;
use crate::types::Language;
use crate::types::Limit;
use crate::types::Path;

impl Language {
    // TODO: 
    pub fn get_compiler(&self) -> Option<Box<dyn Compiler>> {
        unimplemented!()
    }

    pub fn get_source_name(&self)->&'static str{
        unimplemented!()
    }

    pub fn get_binary_name(&self)->Option<&'static str>{
        unimplemented!()
    }

    pub fn get_limit(&self)->Option<Limit>{
        unimplemented!()
    }

    pub fn get_target(&self,target_path:&Path)->(&str, Vec<&str>){
        unimplemented!()
    }

    pub fn get_security_cfg(&self)->Option<&'static str>{
        unimplemented!()
    }
}
