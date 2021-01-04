use clap::{App, Arg, SubCommand};
use diesel::PgConnection;
use std::io::{BufRead, Read};

mod quest;

fn main() {
    let matches = App::new("Paddlers Specification Loader")
        .subcommand(
            SubCommand::with_name("upload-quests")
                .arg(Arg::with_name("INPUT_FILE").required(true).index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("upload-quests") {
        let db = paddlers_shared_lib::establish_connection();
        let file = matches.value_of("INPUT_FILE").unwrap();
        upload_quests(&db, open_file(file));
    }
}

fn open_file(path: &str) -> impl BufRead {
    let f = std::fs::File::open(path).expect("Failed reading file.");
    std::io::BufReader::new(f)
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
            println!("Uploaded {} quests, {} failed.", okay, n-okay)
        }
        Err(e) => eprintln!("Quest parsing failed. {}", e),
    }
}