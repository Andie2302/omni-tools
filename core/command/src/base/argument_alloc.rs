#![cfg(feature = "alloc")]

use alloc::vec::Vec;
use core::fmt;
use core::fmt::{Display, Formatter};
use crate::{Argument, Formatting};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArgumentList<'a> {
    pub arguments: Vec<Argument<'a>>,
    pub formatting: Formatting<'a>,
}
impl Display for ArgumentList<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatting.affix.prefix)?;
        for (i, arg) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, "{}", self.formatting.separator)?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, "{}", self.formatting.affix.postfix)
    }
}