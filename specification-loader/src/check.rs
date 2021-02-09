use paddlers_shared_lib::{
    specification_types::{Scene, SceneIndex},
    strum::VariantNames,
};
use std::path::Path;

pub enum DialogueCheckError {
    Io(std::io::Error),
    InvalidPath(String),
    MissingSceneDefinition(&'static str),
    ParserError(String, String),
}

pub fn check_dialogue_scenes(path: &Path) -> Result<(), DialogueCheckError> {
    if !path.is_dir() {
        return Err(DialogueCheckError::InvalidPath(
            path.to_str().unwrap().to_string(),
        ));
    }
    let mut path = path.to_path_buf();
    path.push("placeholder");
    for scene in SceneIndex::VARIANTS {
        path.set_file_name(scene);
        path.set_extension("ron");
        if !path.exists() {
            return Err(DialogueCheckError::MissingSceneDefinition(scene));
        }
        let reader = super::open_file(&path)?;
        if let Err(parser_error) = ron::de::from_reader::<_, Scene>(reader) {
            return Err(DialogueCheckError::ParserError(
                parser_error.to_string(),
                path.to_str().unwrap().to_string(),
            ));
        }
    }
    Ok(())
}

impl From<std::io::Error> for DialogueCheckError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl std::fmt::Display for DialogueCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogueCheckError::Io(err) => {
                write!(f, "IO Error: {}", err)
            }
            DialogueCheckError::InvalidPath(path) => {
                write!(f, "Invalid directory for scenes: {} ", path)
            }
            DialogueCheckError::ParserError(parser_error, file) => {
                write!(f, "Invalid definition in {}: {}", file, parser_error)
            }
            DialogueCheckError::MissingSceneDefinition(scene) => {
                write!(f, "Scene definition missing for {}", scene)
            }
        }
    }
}
