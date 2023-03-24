use crate::Tokens;
use std::default::default;
use std::fmt::{Display, Formatter};
use std::iter::Map;

auto trait Pun {}
impl<T: Tokens, P: Tokens> !Pun for Punctuated<T, P> {}

pub enum Pair<T: Tokens, P: Tokens> {
    Punct(T, P),
    End(T),
}

impl<T: Tokens, P: Tokens> Pair<T, P> {
    pub fn into_value(self) -> T {
        match self {
            Pair::Punct(t, _) | Pair::End(t) => t,
        }
    }
    pub fn value(&self) -> &T {
        match self {
            Pair::Punct(t, _) | Pair::End(t) => t,
        }
    }
    pub fn value_mut(&mut self) -> &mut T {
        match self {
            Pair::Punct(t, _) | Pair::End(t) => t,
        }
    }
}

pub struct Punctuated<T: Tokens, P: Tokens> {
    segments: Vec<Pair<T, P>>,
}

impl<T: Tokens, P: Tokens> Punctuated<T, P> {
    pub const fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn from_segments(segments: Vec<Pair<T, P>>) -> Self {
        Punctuated { segments }
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

    pub fn stringify(&self) -> String where T: Display, P: Display {
        self.pairs()
            .map(|a| match a {
                Pair::Punct(real, punct) => format!("{}{}", real.to_string(), punct.to_string()),
                Pair::End(real) => real.to_string(),
            })
            .fold(String::new(), |acc, cur| acc + &cur)
    }
}

impl<T: Tokens, P: Tokens> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = Map<<Vec<Pair<T, P>> as IntoIterator>::IntoIter, fn(Pair<T, P>) -> T>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments.into_iter().map(|a| a.into_value())
    }
}

impl<T: Tokens, P: Tokens + Default> FromIterator<T> for Punctuated<T, P> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut punct = Punctuated::new();

        let mut segments: Vec<T> = iter.into_iter().collect();

        if let Some(last) = segments.pop() {
            if !segments.is_empty() {
                punct
                    .segments
                    .extend(segments.into_iter().map(|a| Pair::Punct(a, default())));
            }

            punct.segments.push(Pair::End(last));
        }

        punct
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
            vec![Pair::End(value.into())], // into T
        )
    }
}

impl<T: Tokens + Display, P: Tokens + Display> Display for Punctuated<T, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.stringify())
    }
}

impl<T: Tokens, P: Tokens> crate::private::_Tokens for Punctuated<T, P> {}

impl<T: Tokens, P: Tokens> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self::new()
    }
}
