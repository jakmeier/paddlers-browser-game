use paddlers_shared_lib::story::{story_state::StoryState, story_transitions::StoryTransition};
use paddlers_shared_lib::strum::IntoEnumIterator;

/// Generate a flow chart in dot format from the defined FSM
pub fn generate_dot_story_diagram(out: &mut impl std::io::Write) -> std::io::Result<()> {
    writeln!(out, "digraph MyGraph {{")?;
    writeln!(out, "graph [outputorder=edgesfirst];")?;
    for s in StoryState::iter() {
        state_definition(out, s)?;
    }
    for s in StoryState::iter() {
        state_transitions(out, s)?;
    }
    writeln!(out, "}}")?;
    Ok(())
}
fn state_definition(out: &mut impl std::io::Write, s: StoryState) -> std::io::Result<()> {
    writeln!(out, "{0:?} [label=\"{0}\"]", s)
}
fn state_transitions(out: &mut impl std::io::Write, s: StoryState) -> std::io::Result<()> {
    for transition in s.guards().into_iter() {
        let mut col = "black";
        if s == transition.next_state {
            col = "invis";
        }
        write!(
            out,
            "{:?} -> {:?} [decorate=true, color={}, label=< <B>{:?}</B> ",
            s, transition.next_state, col, transition.trigger
        )?;
        for action in transition.actions.into_iter() {
            write!(out, "<br/>{:?} ", action)?;
        }
        writeln!(out, ">]")?;
    }
    Ok(())
}
