use actix_web::{web, HttpResponse, Responder};
use paddlers_shared_lib::api::statistics::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

trait LogDisplay {
    fn write(&self, f: &mut File) -> Result<(), std::io::Error>;
}

pub(super) fn new_frontend_info(body: web::Json<FrontendRuntimeStatistics>) -> impl Responder {
    if let Some(mut f) = open_file_writer() {
        let res = body.write(&mut f).and_then(|_| write!(f, "\n"));

        if let Err(e) = res {
            println!("Logging to file failed. {}", e);
        }
    }
    HttpResponse::Ok()
}

fn open_file_writer() -> Option<File> {
    let filename = "browser_stats.data";
    let file = OpenOptions::new().append(true).create(true).open(filename);
    match file {
        Err(e) => {
            println!("Opening file {} failed. Reason: {}", filename, e);
            None
        }
        Ok(file) => Some(file),
    }
}

impl LogDisplay for FrontendRuntimeStatistics {
    fn write(&self, f: &mut File) -> Result<(), std::io::Error> {
        write!(f, "{} {} ", self.fps, self.session_duration_s)?;
        self.browser.write(f)
    }
}
impl LogDisplay for BrowserInfo {
    fn write(&self, f: &mut File) -> Result<(), std::io::Error> {
        write!(
            f,
            "{} {} {} {} ",
            self.inner_width, self.inner_height, self.outer_width, self.outer_height
        )?;
        write!(f, "\"{}\"", self.user_agent)
    }
}
