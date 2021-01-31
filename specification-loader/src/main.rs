use clap::{App, Arg, SubCommand};
use diesel::PgConnection;
use std::io::{BufRead, Read, Write};

mod gen;
mod quest;

fn main() {
    let matches = App::new("Paddlers Specification Loader")
        .subcommand(
            SubCommand::with_name("upload-quests")
                .arg(Arg::with_name("INPUT_FILE").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .before_help("Generates some files from specifications.")
                .arg(
                    Arg::with_name("GENERATION_TARGET")
                        .required(true)
                        .index(1)
                        .help("{enum, chart}"),
                )
                .arg(
                    Arg::with_name("OUTPUT_DIR")
                        .required(true)
                        .index(2)
                        .help("Path where generated files will go."),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("upload-quests") {
        let db = paddlers_shared_lib::establish_connection();
        let file = matches.value_of("INPUT_FILE").unwrap();
        upload_quests(&db, open_file(file).unwrap());
    }
    if let Some(matches) = matches.subcommand_matches("generate") {
        match matches.value_of("GENERATION_TARGET").unwrap() {
            "enum" => {
                let path = matches.value_of("OUTPUT_DIR").unwrap();
                let backup = std::fs::read_to_string(path.to_owned() + "/quest.rs").unwrap();
                let mut quest_rs = write_file(&(path.to_owned() + "/quest.rs")).unwrap();
                if let Err(e) = gen::generate_quest_enum(&mut quest_rs) {
                    eprintln!("\x1b[031mFailed generating quest enum: {}\x1b[0m", e);
                    write!(quest_rs, "{}", backup).unwrap();
                }
            }
            "chart" => {
                let path = matches.value_of("OUTPUT_DIR").unwrap();
                let mut story_dot = write_file(&(path.to_owned() + "/story.dot")).unwrap();
                gen::generate_dot_story_diagram(&mut story_dot).unwrap();
            }
            target => {
                eprintln!("Unknown generation target: {}", target)
            }
        }
    }
}

fn upload_quests(db: &PgConnection, input: impl Read) {
    match ron::de::from_reader::<_, Vec<quest::QuestDefinition>>(input) {
        Ok(quests) => {
            let n = quests.len();
            let mut okay = n;
            for quest in quests {
                if let Err(e) = quest.upload(db) {
                    eprintln!("{}", e);
                    okay -= 1;
                }
            }
            println!("Uploaded {} quests, {} failed.", okay, n - okay)
        }
        Err(e) => eprintln!("Quest parsing failed. {}", e),
    }
}

fn open_file(path: &str) -> std::io::Result<impl BufRead> {
    let f = std::fs::File::open(path)?;
    Ok(std::io::BufReader::new(f))
}
fn write_file(path: &str) -> std::io::Result<std::io::BufWriter<impl std::io::Write>> {
    let f = std::fs::File::create(path)?;
    Ok(std::io::BufWriter::new(f))
}

fn read_quests_from_file(path: &'static str) -> Result<Vec<quest::QuestDefinition>, String> {
    ron::de::from_reader(open_file(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}
