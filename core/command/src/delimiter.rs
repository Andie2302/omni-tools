use std::borrow::Cow;
use std::marker::PhantomData;

// 1. Sealed Trait Pattern: Verhindert, dass externe Crate den Trait unkontrolliert implementieren
mod private {
    pub trait Sealed {}
}

pub trait DelimiterType: private::Sealed {}

macro_rules! impl_delimiter_type {
    ($($t:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $t;
            impl private::Sealed for $t {}
            impl DelimiterType for $t {}
        )*
    };
}

impl_delimiter_type!(
    DelimiterPrefix,
    DelimiterSeparator,
    DelimiterPostfix,
    DelimiterKey,
    DelimiterValue
);

// 2. Transparente Struktur mit Default-Lifetimes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelimiterString<'a, T: DelimiterType> {
    text: &'a str,
    prefix: Option<&'a str>,
    suffix: Option<&'a str>,
    _phantom: PhantomData<fn() -> T>, 
}

impl<'a, T: DelimiterType> DelimiterString<'a, T> {
    #[inline]
    pub fn new(text: &'a str, prefix: Option<&'a str>, suffix: Option<&'a str>) -> Self {
        Self {
            text,
            prefix,
            suffix,
            _phantom: PhantomData,
        }
    }
    
    pub fn delimit(&self) -> Cow<'a, str> {
        match (self.prefix, self.suffix) {
            (None | Some(""), None | Some("")) => Cow::Borrowed(self.text),
            (Some(p), None | Some("")) => Cow::Owned(format!("{p}{}", self.text)),
            (None | Some(""), Some(s)) => Cow::Owned(format!("{}{s}", self.text)),
            (Some(p), Some(s)) => Cow::Owned(format!("{p}{}{s}", self.text)),
        }
    }
}

