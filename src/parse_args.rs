use clap::{App, Arg};
use std::env;

pub struct Args {
    path: String,
}

impl Args {
    pub fn get_path(&self) -> &String {
        &self.path
    }
}

pub fn get_args() -> Args {
    let app = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(Arg::with_name("path")
        .help("complete file/dir path.")
    );

    let matches = app.get_matches();
    Args {
        path: match matches.value_of("path") {
            Some(s) => s.to_string(),
            None => "".to_string()
        }
    }
}