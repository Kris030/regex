#![allow(clippy::needless_range_loop)]

use compiler::RegexCompiler;
use engine::EngineState;

pub mod compiler;
pub mod engine;

pub fn build_states(re: &str) -> Vec<RegexState> {
    RegexCompiler::new(re).compile()
}

#[derive(Debug, Clone, Copy)]
pub enum MatchType {
    Is,
    IsNot,
}

#[derive(Debug, Clone)]
pub enum OneOrMore<T> {
    One(T),
    More(Vec<T>),
}

#[derive(Debug, Clone)]
pub struct CharMatcher {
    ty: MatchType,
    elems: OneOrMore<char>,
}

impl std::fmt::Display for CharMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.elems {
            OneOrMore::One(c) => {
                if let MatchType::Is = self.ty {
                    write!(f, "{c}")
                } else {
                    write!(f, "[^{c}]")
                }
            }

            OneOrMore::More(v) => {
                if let MatchType::Is = self.ty {
                    write!(f, "[")?;
                } else {
                    write!(f, "[^")?;
                }

                for c in 0..(v.len() - 1) {
                    write!(f, "{}, ", v[c])?;
                }

                write!(f, "{}]", v[v.len() - 1])
            }
        }
    }
}

impl CharMatcher {
    pub fn new(ty: MatchType, elems: OneOrMore<char>) -> Self {
        Self { ty, elems }
    }

    pub fn matches(&self, chr: char) -> bool {
        let m = match &self.elems {
            OneOrMore::One(o) => *o == chr,
            OneOrMore::More(v) => v.contains(&chr),
        };

        match self.ty {
            MatchType::Is => m,
            MatchType::IsNot => !m,
        }
    }
}

#[derive(Debug)]
pub struct RegexState {
    pub next: Vec<(CharMatcher, EngineState)>,
}
