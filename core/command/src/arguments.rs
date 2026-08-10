//#![allow(dead_code)]

use crate::argument::Argument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments<'a> {
    items: Vec<Argument<'a>>,
    item_separator: &'a str,
    enclosing_delimiter: Option<(&'a str, &'a str)>,
}

impl<'a> Arguments<'a> {
    pub fn new(item_separator: &'a str) -> Self {
        Self {
            items: Vec::new(),
            item_separator,
            enclosing_delimiter: None,
        }
    }

    pub fn push(&mut self, argument: Argument<'a>) {
        self.items.push(argument);
    }

    pub fn with_argument(mut self, argument: Argument<'a>) -> Self {
        self.items.push(argument);
        self
    }

    pub fn set_item_separator(&mut self, separator: &'a str) {
        self.item_separator = separator;
    }

    pub fn with_enclosing_delimiter(mut self, start: &'a str, end: &'a str) -> Self {
        self.enclosing_delimiter = Some((start, end));
        self
    }

    pub fn items(&self) -> &[Argument<'a>] {
        &self.items
    }

    pub fn to_arg_strings(&self) -> Vec<String> {
        self.items.iter().map(|arg| arg.render()).collect()
    }

    pub fn render(&self) -> String {
        if self.items.is_empty() {
            return String::new();
        }

        let joined = self
            .items
            .iter()
            .map(|arg| arg.render())
            .collect::<Vec<String>>()
            .join(self.item_separator);

        if let Some((start, end)) = self.enclosing_delimiter {
            format!("{start}{joined}{end}")
        } else {
            joined
        }
    }
}

impl<'a> Default for Arguments<'a> {
    fn default() -> Self {
        Self::new(" ")
    }
}