use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontendRuntimeStatistics {
    pub browser: BrowserInfo,
    pub session_duration_s: i64,
    pub fps: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowserInfo {
    pub user_agent: String,
    pub inner_width: i32,
    pub inner_height: i32,
    pub outer_width: i32,
    pub outer_height: i32,
}
