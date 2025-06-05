#![allow(unused)]
use std::borrow::Cow;

pub trait MaybeReplaceExt<'a> {
    fn maybe_replace(self, needle: &str, replacement: &str) -> Cow<'a, str>;
    fn maybe_replace_closure<F>(self, needle: &str, replacement: F) -> Cow<'a, str>
    where
        F: FnOnce() -> String;
}

impl<'a> MaybeReplaceExt<'a> for &'a str {
    fn maybe_replace(self, needle: &str, replacement: &str) -> Cow<'a, str> {
        // Assumes that searching twice is better than unconditionally allocating
        if self.contains(needle) {
            self.replace(needle, replacement).into()
        } else {
            self.into()
        }
    }

    fn maybe_replace_closure<F>(self, needle: &str, replacement: F) -> Cow<'a, str>
    where
        F: FnOnce() -> String,
    {
        if self.contains(needle) {
            self.replace(needle, &replacement()).into()
        } else {
            self.into()
        }
    }
}

impl<'a> MaybeReplaceExt<'a> for Cow<'a, str> {
    fn maybe_replace(self, needle: &str, replacement: &str) -> Cow<'a, str> {
        // Assumes that searching twice is better than unconditionally allocating
        if self.contains(needle) {
            self.replace(needle, replacement).into()
        } else {
            self
        }
    }

    fn maybe_replace_closure<F>(self, needle: &str, replacement: F) -> Cow<'a, str>
    where
        F: FnOnce() -> String,
    {
        if self.contains(needle) {
            self.replace(needle, &replacement()).into()
        } else {
            self
        }
    }
}
