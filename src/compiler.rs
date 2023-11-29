use crate::{engine::EngineState, CharMatcher, MatchType, OneOrMore, RegexState};

pub struct RegexCompiler<'re> {
    src: std::iter::Peekable<std::str::Chars<'re>>,
    states: Vec<RegexState>,
}

impl<'re> RegexCompiler<'re> {
    pub fn new(regex: &'re str) -> Self {
        Self {
            src: regex.chars().peekable(),
            states: vec![],
        }
    }

    fn parse_group(&mut self) {
        let mut curr_state = self.states.len();
        let c = match self.src.next() {
            Some('(') => {
                let first_char = *self.src.peek().expect("unclosed group...");

                while self.src.peek().is_some_and(|c| *c != ')') {
                    self.parse_group();
                }

                if self.src.next() != Some(')') {
                    panic!("unmatched paren");
                }
                curr_state += 1;
                first_char
            }

            Some(c) => c,

            None => return,
        };
        let next_state = self.states.len() + 1;

        let next_state = if self.src.peek().is_some() {
            EngineState::State(next_state)
        } else {
            EngineState::Done
        };

        let new_state = match self.parse_modifiers() {
            Modifier::ZeroPlus => todo!(),
            Modifier::Optional => todo!(),

            Modifier::OnePlus => {
                let next = if let Some(next) = self.src.next() {
                    (
                        CharMatcher::new(MatchType::Is, OneOrMore::One(next)),
                        next_state,
                    )
                } else {
                    (
                        CharMatcher::new(MatchType::IsNot, OneOrMore::One(c)),
                        EngineState::Done,
                    )
                };

                RegexState {
                    next: vec![
                        (
                            CharMatcher::new(MatchType::Is, OneOrMore::One(c)),
                            EngineState::State(curr_state),
                        ),
                        next,
                    ],
                }
            }

            Modifier::None => RegexState {
                next: vec![(
                    CharMatcher::new(MatchType::Is, OneOrMore::One(c)),
                    next_state,
                )],
            },
        };

        self.states.push(new_state);
    }

    fn parse_modifiers(&mut self) -> Modifier {
        let md = match self.src.peek() {
            Some('?') => Modifier::Optional,
            Some('*') => Modifier::ZeroPlus,
            Some('+') => Modifier::OnePlus,
            _ => return Modifier::None,
        };

        self.src.next();

        md
    }

    pub fn compile(mut self) -> Vec<RegexState> {
        while self.src.peek().is_some() {
            self.parse_group();
        }

        let end = self.states.len();
        for s in 0..end {
            for (_, s) in &mut self.states[s].next {
                if *s == EngineState::State(end) {
                    *s = EngineState::Done;
                }
            }
        }
        self.states
    }
}

enum Modifier {
    ZeroPlus,
    Optional,
    OnePlus,
    None,
}
