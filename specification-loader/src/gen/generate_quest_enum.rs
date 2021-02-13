use crate::quest::QuestDefinition;
use crate::*;
use heck::CamelCase;

const QUESTS_0_2_1: &'static str = "../specification/quests.0.2.1.ron";

pub fn generate_quest_enum(out: &mut impl std::io::Write) -> Result<(), String> {
    let parsed_quests = all_quests()?;
    super::generation_note(out).map_err(|e| e.to_string())?;
    generate_type(out, &parsed_quests).map_err(|e| e.to_string())?;
    generate_impl_key(out, &parsed_quests).map_err(|e| e.to_string())?;
    generate_impl_parse(out, &parsed_quests).map_err(|e| e.to_string())?;
    Ok(())
}

fn generate_type(
    out: &mut impl std::io::Write,
    parsed_quests: &[QuestDefinition],
) -> std::io::Result<()> {
    writeln!(out, "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]")?;
    writeln!(out, "pub enum QuestName {{")?;
    let indent = "    ";
    for quest in parsed_quests {
        writeln!(out, "{}{},", indent, quest.quest_key.to_camel_case())?;
    }
    writeln!(out, "}}")?;
    Ok(())
}
fn generate_impl_key(
    out: &mut impl std::io::Write,
    parsed_quests: &[QuestDefinition],
) -> std::io::Result<()> {
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
            for quest in parsed_quests {
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

fn all_quests() -> Result<Vec<QuestDefinition>, String> {
    let mut out = vec![];
    out.append(&mut read_quests_from_file(QUESTS_0_2_1)?);
    Ok(out)
}

fn generate_impl_parse(
    out: &mut impl std::io::Write,
    parsed_quests: &[QuestDefinition],
) -> std::io::Result<()> {
    writeln!(out, "impl std::str::FromStr for QuestName {{")?;
    let indent = "    ";
    writeln!(out, "{}type Err = ();", indent)?;
    writeln!(
        out,
        "{}fn from_str(s: &str) -> Result<Self, Self::Err> {{",
        indent
    )?;
    {
        let indent = "        ";
        writeln!(out, "{}match s {{", indent)?;
        {
            let indent = "            ";
            for quest in parsed_quests {
                writeln!(
                    out,
                    "{}\"{}\" => Ok(Self::{}),",
                    indent,
                    quest.quest_key,
                    quest.quest_key.to_camel_case(),
                )?;
            }
            writeln!(out, "{}_ => Err(()),", indent)?;
        }
        writeln!(out, "{}}}", indent)?;
    }
    writeln!(out, "{}}}", indent)?;
    writeln!(out, "}}")?;
    Ok(())
}
