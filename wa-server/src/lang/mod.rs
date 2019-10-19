mod gcc;
mod java;
mod python3;
mod rustc;

use gcc::Gcc;

use crate::types::Language;
use crate::types::LanguageBroker;

impl Language {
    pub fn get_broker(self) -> Box<dyn LanguageBroker> {
        use Language::*;

        match self {
            C11 => Box::new(Gcc {
                is_cpp: false,
                std: "c11",
            }),
            C89 => Box::new(Gcc {
                is_cpp: false,
                std: "c89",
            }),
            C99 => Box::new(Gcc {
                is_cpp: false,
                std: "c99",
            }),
            Cpp11 => Box::new(Gcc {
                is_cpp: true,
                std: "c++11",
            }),
            Cpp14 => Box::new(Gcc {
                is_cpp: true,
                std: "c++14",
            }),
            Cpp17 => Box::new(Gcc {
                is_cpp: true,
                std: "c++17",
            }),
            Rust => Box::new(rustc::Rustc),
            Java => Box::new(java::Java),
            Python3 => Box::new(python3::Python3),
        }
    }
}
