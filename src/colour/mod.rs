mod color;

pub use color::*;

use std::convert::From;
use std::fmt;
use std::ops::Deref;
use std::string::String;

pub struct FormattedOutput {
    value: String,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

pub trait Colour {
    fn red(self) -> FormattedOutput 
    where
        Self: Sized,
    {
        self.color(Color::Red)
    }
}

impl Default for FormattedOutput {
    fn default() -> Self {
        FormattedOutput {
            value: String::default(),
            fg_color: None,
            bg_color: None,
        }
    }
}

impl<'a> From<&'a str> for FormattedOutput {
    fn from(s: &'a str) -> Self {
        FormattedOutput {
            value: String::from(s),
            ..FormattedOutput::default()
        }
    }
}

impl Colour for FormattedOutput {
    fn color<S: Into<Color>>(mut self, color: S) -> FormattedOutput {
        self.fg_color = Some(color.into());
        self
    }
}

impl<'a> Colour for &'a str {
    fn color<S: Into<Color>>(self, color: S) -> FormattedOutput {
        FormattedOutput {
            value: String.from(self),
            fg_color: Some(color.into()),
            ..FormattedOutput::default(),
        }
    }
}