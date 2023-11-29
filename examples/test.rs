use regex::{
    build_states,
    engine::{EngineState, RegexEngine},
    RegexState,
};

fn main() {
    let s = "Hellowowowowo World!";

    let states = build_states("Hello(wo)+ World!");

    write_states_graph(&states, &mut std::fs::File::create("fsm.dot").unwrap()).unwrap();

    let eng = RegexEngine::new(s, states);
    let res = eng.run();

    println!("{}", if res { "matched!" } else { "failed :((" });
}

fn write_states_graph(states: &[RegexState], w: &mut impl std::io::Write) -> std::io::Result<()> {
    write!(w, "digraph G {{")?;

    for (i, s) in states.iter().enumerate() {
        write!(w, "s{i}[label=\"{i}\"];")?;

        for (c, ns) in &s.next {
            let ns = if *ns == EngineState::State(states.len()) {
                EngineState::Done
            } else {
                *ns
            };

            match ns {
                EngineState::State(ns) => write!(w, "s{i} -> s{ns} [label=\"{c}\"];")?,
                EngineState::Done => write!(w, "s{i} -> end [label=\"{c}\"];")?,

                _ => (),
            }
        }
    }

    write!(w, "}}")
}
