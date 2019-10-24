use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

pub struct CowOsStr<'a>(Cow<'a, OsStr>);

impl<'a> AsRef<OsStr> for CowOsStr<'a> {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl<'a, T> From<&'a T> for CowOsStr<'a>
where
    T: AsRef<OsStr> + ?Sized,
{
    fn from(s: &'a T) -> Self {
        Self(Cow::Borrowed(s.as_ref()))
    }
}

macro_rules! impl_CowOsStr_From{
    {$tp:tt}=>{
        impl<'a> From<$tp> for CowOsStr<'a> {
            fn from(s: $tp) -> Self {
                Self(Cow::Owned(s.into()))
            }
        }
    }
}

impl_CowOsStr_From! {String}
impl_CowOsStr_From! {PathBuf}
impl_CowOsStr_From! {OsString}
