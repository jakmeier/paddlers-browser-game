use paddle::NutsCheck;
use paddlers_shared_lib::specification_types::*;
use std::collections::HashMap;

use crate::prelude::{PadlError, PadlErrorCode, PadlResult};

/// Responsible for loading and caching scene data
pub struct SceneLoader {
    data: HashMap<SceneIndex, SceneData>,
}
pub(super) struct SceneResponse {
    pub scene: Scene,
    pub index: SceneIndex,
}

impl Default for SceneLoader {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
enum SceneData {
    RequestPending,
    Loaded(Scene),
}

impl SceneLoader {
    pub fn get(&mut self, index: SceneIndex) -> Option<&Scene> {
        let data = self.data.entry(index).or_insert_with(|| {
            spawn_scene_request(index);
            SceneData::RequestPending
        });

        if let SceneData::Loaded(scene) = data {
            Some(scene)
        } else {
            None
        }
    }
    pub(super) fn add(&mut self, s: SceneResponse) {
        self.data.insert(s.index, SceneData::Loaded(s.scene));
    }
}

fn spawn_scene_request(index: SceneIndex) {
    let future = async move {
        if let Some(scene) = http_fetch_scene(index).await.nuts_check() {
            let msg = SceneResponse { index, scene };
            paddle::send::<_, super::DialogueFrame>(msg);
        }
    };
    wasm_bindgen_futures::spawn_local(future);
}

async fn http_fetch_scene(i: SceneIndex) -> PadlResult<Scene> {
    let path = i.scene_path();
    let binary = paddle::load_file(&path).await?;
    ron::de::from_bytes(&binary)
        .map_err(|parser_error| PadlError::dev_err(PadlErrorCode::RonParseError(parser_error)))
}
