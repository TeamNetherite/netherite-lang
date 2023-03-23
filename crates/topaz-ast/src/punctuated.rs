use crate::{_Tokens, token};
use std::iter::Map;

pub enum Pair<T: _Tokens, P: _Tokens> {
    Punct(T, P),
    End(T),
}

impl<T: _Tokens, P: _Tokens> Pair<T, P> {
    pub fn into_value(self) -> T {
        match self { Pair::Punct(t, _) | Pair::End(t) => t }
    }

    pub fn value(&self) -> &T {
        match self { Pair::Punct(t, _) | Pair::End(t) => t }
    }

    pub fn value_mut(&mut self) -> &mut T {
        match self { Pair::Punct(t, _) | Pair::End(t) => t }
    }
}

pub struct Punctuated<T: _Tokens, P: _Tokens> {
    segments: Vec<Pair<T, P>>,
}

impl<T: _Tokens, P: _Tokens> Punctuated<T, P> {
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn auto_push(&mut self, thing: T)
    where
        P: Default,
    {
        self.segments.push(Pair::Punct(thing, P::default()))
    }

    pub fn end(&mut self, thing: T) -> Result<(), T> {
        if self
            .segments
            .last()
            .is_some_and(|a| matches!(a, Pair::End(_)))
        {
            Err(thing)
        } else {
            Ok(self.segments.push(Pair::End(thing)))
        }
    }

    pub fn into_pairs(self) -> <Vec<Pair<T, P>> as IntoIterator>::IntoIter {
        self.segments.into_iter()
    }
    pub fn pairs(&self) -> impl Iterator<Item = &Pair<T, P>> {
        self.segments.iter()
    }
    pub fn pairs_mut(&mut self) -> impl Iterator<Item = &mut Pair<T, P>> {
        self.segments.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (self.segments.iter()).map(|a: &Pair<T, P>| a.value())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        (self.segments.iter_mut()).map(|a: &mut Pair<T, P>| a.value_mut())
    }
}

impl<T: _Tokens, P: _Tokens> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = Map<<Vec<Pair<T, P>> as IntoIterator>::IntoIter, fn(Pair<T, P>) -> T>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter().map(|a| a.into_value())
    }
}

impl<T: _Tokens, P: _Tokens + Default> FromIterator<T> for Punctuated<T, P> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut punct = Punctuated::new();

        let mut vec: Vec<T> = iter.into_iter().collect();

        if let Some(last) = vec.pop() {
            for i in vec {
                punct.auto_push(i)
            }

            punct.segments.push(Pair::End(last));
        }

        punct
    }
}

impl<T: _Tokens, P: _Tokens> _Tokens for Punctuated<T, P> {}

impl<T: _Tokens, P: _Tokens> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self::new()
    }
}
