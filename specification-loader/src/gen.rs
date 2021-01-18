mod generate_quest_enum;

pub use generate_quest_enum::*;

pub fn generation_note(out: &mut impl std::io::Write) -> std::io::Result<()> {
    writeln!(
        out,
        "//! This module has been auto-generate using specification loader."
    )?;
    Ok(())
}
