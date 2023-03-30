use crate::Tokens;
use std::default::default;
use std::fmt::{Display, Formatter};
use std::iter::Map;
use std::marker::PhantomData;

// i was brainded
auto trait Pun {}
impl<T: Tokens, P: Tokens> !Pun for Punctuated<T, P> {}

#[derive(Tokens)]
pub struct Punctuated<T: Tokens, P: Tokens> {
    segments: Vec<T>,
    phantom: PhantomData<P>,
}

impl<T: Tokens, P: Tokens> Punctuated<T, P> {
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn from_segments(segments: Vec<T>) -> Self {
        Punctuated {
            segments,
            phantom: PhantomData,
        }
    }

    pub fn last(&self) -> Option<&T> {
        self.segments.last()
    }

    pub fn push(&mut self, thing: T)
    where
        P: Default,
    {
        self.segments.push(thing)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (self.segments.iter())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        (self.segments.iter_mut())
    }

    pub fn stringify(&self) -> String
    where
        T: Display,
    {
        self.iter()
            .fold(String::new(), |acc, current| acc + &current.to_string())
    }
}

impl<T: Tokens, P: Tokens> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter()
    }
}

impl<T: Tokens, P: Tokens + Default> FromIterator<T> for Punctuated<T, P> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Punctuated::from_segments(iter.into_iter().collect())
    }
}

impl<I, T, P> From<I> for Punctuated<T, P>
where
    I: Pun + Into<T>,
    T: Tokens,
    P: Tokens,
{
    fn from(value: I) -> Self {
        Punctuated::from_segments(
            vec![value.into()], // into T
        )
    }
}

impl<T: Tokens + Display, P: Tokens + Display> Display for Punctuated<T, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.stringify())
    }
}

impl<T: Tokens, P: Tokens> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self::new()
    }
}
