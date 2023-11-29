use crate::RegexState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    State(usize),
    Failed,
    Done,
}

pub struct RegexEngine<'s> {
    src: std::iter::Peekable<std::str::Chars<'s>>,

    states: Vec<RegexState>,
    curr_state: EngineState,
}

impl<'s> RegexEngine<'s> {
    pub fn new(src: &'s str, states: Vec<RegexState>) -> Self {
        Self {
            curr_state: EngineState::State(0),
            src: src.chars().peekable(),
            states,
        }
    }

    pub fn run(mut self) -> bool {
        while let EngineState::State(state) = self.curr_state {
            let Some(chr) = self.src.peek().copied() else {
                return false;
            };

            let state = &self.states[state];
            let new_state = state
                .next
                .iter()
                .find_map(|(c, next)| if c.matches(chr) { Some(next) } else { None })
                .copied();

            if let Some(new_state) = new_state {
                self.curr_state = new_state;
                self.src.next();
            } else {
                return false;
            }
        }

        self.curr_state == EngineState::Done
    }
}
