use crate::quest::QuestDefinition;
use crate::*;
use heck::CamelCase;

const QUESTS_0_2_1: &'static str = "../specification/quests.0.2.1.ron";

pub fn generate_quest_enum(out: &mut impl std::io::Write) -> std::io::Result<()> {
    super::generation_note(out)?;
    generate_type(out)?;
    generate_impl_key(out)?;
    Ok(())
}

fn generate_type(out: &mut impl std::io::Write) -> std::io::Result<()> {
    writeln!(out, "#[derive(Clone,Copy,Debug)]")?;
    writeln!(out, "pub enum QuestName {{")?;
    let indent = "    ";
    for quest in all_quests() {
        writeln!(out, "{}{},", indent, quest.quest_key.to_camel_case())?;
    }
    writeln!(out, "}}")?;
    Ok(())
}
fn generate_impl_key(out: &mut impl std::io::Write) -> std::io::Result<()> {
    writeln!(out, "impl QuestName {{")?;
    let indent = "    ";
    writeln!(
        out,
        "{}pub fn unique_string(&self) -> &'static str {{",
        indent
    )?;
    {
        let indent = "        ";
        writeln!(out, "{}match self {{", indent)?;
        {
            let indent = "            ";
            for quest in all_quests() {
                writeln!(
                    out,
                    "{}Self::{} => \"{}\",",
                    indent,
                    quest.quest_key.to_camel_case(),
                    quest.quest_key
                )?;
            }
        }
        writeln!(out, "{}}}", indent)?;
    }
    writeln!(out, "{}}}", indent)?;
    writeln!(out, "}}")?;
    Ok(())
}

fn all_quests() -> Vec<QuestDefinition> {
    let mut out = vec![];
    out.append(&mut read_quests_from_file(QUESTS_0_2_1).unwrap());
    out
}
