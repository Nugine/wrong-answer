mod gcc;
pub use gcc::Gcc;

use crate::types::Language;
use crate::types::LanguageBroker;

impl Language {
    pub fn get_broker(self) -> Box<dyn LanguageBroker> {
        unimplemented!()
    }
}
